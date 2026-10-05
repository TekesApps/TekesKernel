//! Launch-scoped credentials supplied by the owning application.
//! Only explicitly bound variables are read; there is no persistent fallback.

use std::collections::BTreeMap;
use zeroize::Zeroize;

use crate::{SecretRecord, SecretResolution, SecretStore, SecretStoreError};

pub struct EnvironmentSecretStore {
    records: BTreeMap<String, SecretRecord>,
}

impl EnvironmentSecretStore {
    pub fn capture(bindings: &BTreeMap<String, String>) -> Result<Self, SecretStoreError> {
        Self::capture_with(bindings, |name| {
            std::env::var(name).map_err(|_| SecretStoreError::Unavailable)
        })
    }

    fn capture_with(
        bindings: &BTreeMap<String, String>,
        mut read: impl FnMut(&str) -> Result<String, SecretStoreError>,
    ) -> Result<Self, SecretStoreError> {
        let mut records = BTreeMap::new();
        for (id, name) in bindings {
            super::secret_store::validate_credential_id(id)?;
            if name.is_empty()
                || !name.bytes().enumerate().all(|(index, byte)| {
                    byte == b'_'
                        || byte.is_ascii_alphabetic()
                        || (index > 0 && byte.is_ascii_digit())
                })
            {
                return Err(SecretStoreError::InvalidId);
            }
            let record = SecretRecord::Active {
                generation: 1,
                material: read(name)?,
            };
            let mut encoded = record.encode()?;
            encoded.zeroize();
            records.insert(id.clone(), record);
        }
        Ok(Self { records })
    }
}

impl SecretStore for EnvironmentSecretStore {
    fn resolve(&self, credential_id: &str) -> Result<SecretResolution, SecretStoreError> {
        super::secret_store::validate_credential_id(credential_id)?;
        Ok(self
            .records
            .get(credential_id)
            .cloned()
            .map_or(SecretResolution::NotFound, SecretResolution::Active))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn captures_only_explicit_bindings_and_never_falls_back() {
        let bindings = [("provider-key".into(), "TEKES_TEST_KEY".into())].into();
        let mut reads = Vec::new();
        let store = EnvironmentSecretStore::capture_with(&bindings, |name| {
            reads.push(name.to_owned());
            Ok("synthetic-secret".into())
        })
        .unwrap();
        assert_eq!(reads, ["TEKES_TEST_KEY"]);
        assert!(matches!(
            store.resolve("provider-key").unwrap(),
            SecretResolution::Active(_)
        ));
        assert_eq!(
            store.resolve("unselected-key").unwrap(),
            SecretResolution::NotFound
        );
    }

    #[test]
    fn missing_or_malformed_launch_credentials_fail_closed() {
        let bindings = [("provider-key".into(), "TEKES_TEST_KEY".into())].into();
        assert!(
            EnvironmentSecretStore::capture_with(&bindings, |_| Err(SecretStoreError::Unavailable))
                .is_err()
        );
        assert!(EnvironmentSecretStore::capture_with(&bindings, |_| Ok("".into())).is_err());
        let bindings = [("provider-key".into(), "1INVALID".into())].into();
        assert!(
            EnvironmentSecretStore::capture_with(&bindings, |_| panic!(
                "must validate before reading"
            ))
            .is_err()
        );
    }
}
