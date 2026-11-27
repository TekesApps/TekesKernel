//! Compile a value into the existing immutable launch format. No management
//! registry is read or published here.
use std::path::Path;

use crate::{ConfigRepository, ConfigSnapshot, ProfileError};
pub use schema::ExecutionConfig;

/// Opaque marker type proving an `ExecutionConfig` has been validated and
/// canonicalized against a `ConfigRepository` in the current process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedExecution(ExecutionConfig);

impl NormalizedExecution {
    #[must_use]
    pub const fn as_execution(&self) -> &ExecutionConfig {
        &self.0
    }

    #[must_use]
    pub fn into_execution(self) -> ExecutionConfig {
        self.0
    }
}

std::thread_local! {
    static NORMALIZE_CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
    static CANONICALIZE_CALLS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[doc(hidden)]
pub fn reset_execution_metrics() {
    NORMALIZE_CALLS.with(|c| c.set(0));
    CANONICALIZE_CALLS.with(|c| c.set(0));
}

#[doc(hidden)]
#[must_use]
pub fn execution_metrics() -> (u64, u64) {
    (
        NORMALIZE_CALLS.with(std::cell::Cell::get),
        CANONICALIZE_CALLS.with(std::cell::Cell::get),
    )
}

impl ConfigRepository {
    /// Canonicalize host input before recording it in the session ledger.
    pub fn normalize_execution(
        &self,
        input: &ExecutionConfig,
    ) -> Result<ExecutionConfig, ProfileError> {
        NORMALIZE_CALLS.with(|c| c.set(c.get() + 1));
        input
            .validate()
            .map_err(|reason| invalid(reason.to_owned()))?;
        let canonical = |path: &str| -> Result<String, ProfileError> {
            CANONICALIZE_CALLS.with(|c| c.set(c.get() + 1));
            let value = std::fs::canonicalize(path)?;
            if !value.is_dir()
                || value.parent().is_none()
                || tools::protected_root_containing(
                    &tools::kernel_protected_roots(self.root()),
                    &value,
                )
                .is_some()
            {
                return Err(invalid(format!(
                    "execution root is not an admissible directory: {path}"
                )));
            }
            value
                .into_os_string()
                .into_string()
                .map_err(|_| invalid("execution root must be UTF-8".to_owned()))
        };
        let roots = |values: &[String]| -> Result<Vec<String>, ProfileError> {
            let mut values = values
                .iter()
                .map(|value| canonical(value))
                .collect::<Result<Vec<_>, _>>()?;
            values.sort();
            values.dedup();
            Ok(values)
        };
        let mut output = input.clone();
        output.cwd = canonical(&input.cwd)?;
        output.read_roots = roots(&input.read_roots)?;
        output.read_roots.retain(|root| root != &output.cwd);
        output.writable_roots = roots(&input.writable_roots)?;
        output.toolchain_roots = roots(&input.toolchain_roots)?;
        if output.toolchain_roots.iter().any(|root| root.contains(':')) {
            return Err(invalid(
                "toolchain root cannot contain PATH separator".to_owned(),
            ));
        }
        if output.writable_roots.iter().any(|root| {
            !std::iter::once(&output.cwd)
                .chain(&output.read_roots)
                .any(|base| Path::new(root).starts_with(base))
        }) {
            return Err(invalid(
                "writable root is outside execution read roots".to_owned(),
            ));
        }
        output.allowed_tools.sort();
        output.allowed_tools.dedup();
        Ok(output)
    }

    /// Canonicalize and validate host input once, returning a proof wrapper so
    /// immediate snapshot resolution can skip a duplicate canonicalization pass.
    pub fn normalize_execution_checked(
        &self,
        input: &ExecutionConfig,
    ) -> Result<NormalizedExecution, ProfileError> {
        self.normalize_execution(input).map(NormalizedExecution)
    }

    /// Resolve a config snapshot from an execution already normalized in this
    /// flow without repeating directory canonicalization.
    pub fn resolve_normalized_execution(
        &self,
        address: &str,
        execution: &NormalizedExecution,
        session_folder: Option<&Path>,
    ) -> Result<ConfigSnapshot, ProfileError> {
        self.resolve_execution_snapshot(address, &execution.0, session_folder)
    }

    /// Reuse provider/settings and the frozen launch format, without resolving
    /// Environment, Folder catalog or authored workspace documents.
    pub fn resolve_execution(
        &self,
        address: &str,
        execution: &ExecutionConfig,
        session_folder: Option<&Path>,
    ) -> Result<ConfigSnapshot, ProfileError> {
        if self.normalize_execution(execution)? != *execution {
            return Err(invalid(
                "recorded execution roots are no longer canonical".to_owned(),
            ));
        }
        self.resolve_execution_snapshot(address, execution, session_folder)
    }
}

fn invalid(reason: String) -> ProfileError {
    ProfileError::InvalidSchema {
        path: "<execution>".into(),
        reason,
    }
}
