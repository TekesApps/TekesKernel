use std::fmt;
use std::sync::Arc;

use axum::http::HeaderMap;
use axum::http::header::AUTHORIZATION;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use subtle::ConstantTimeEq;
use zeroize::{Zeroize, Zeroizing};

const TOKEN_BYTES: usize = 32;

/// One endpoint bearer supplied to the supervisor by its launcher.
///
/// The transport owns only in-process comparison. Credential storage and
/// rotation remain the launching application's responsibilities.
#[derive(Clone)]
pub struct BearerToken(Arc<SecretBytes>);

struct SecretBytes([u8; TOKEN_BYTES]);

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl fmt::Debug for BearerToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("BearerToken([REDACTED])")
    }
}

impl BearerToken {
    #[must_use]
    pub fn new(bytes: [u8; TOKEN_BYTES]) -> Self {
        Self(Arc::new(SecretBytes(bytes)))
    }

    pub(crate) fn authenticates(&self, headers: &HeaderMap) -> bool {
        let mut values = headers.get_all(AUTHORIZATION).iter();
        let Some(value) = values.next() else {
            return false;
        };
        if values.next().is_some() {
            return false;
        }
        let Ok(value) = value.to_str() else {
            return false;
        };
        let Some(encoded) = value.strip_prefix("Bearer ") else {
            return false;
        };
        let Ok(decoded) = URL_SAFE_NO_PAD.decode(encoded).map(Zeroizing::new) else {
            return false;
        };
        if decoded.len() != TOKEN_BYTES {
            return false;
        }
        bool::from(self.0.0.as_slice().ct_eq(decoded.as_slice()))
    }
}
