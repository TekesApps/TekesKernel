//! Compile a value into the existing immutable launch format. No management
//! registry is read or published here.
use std::path::Path;

use crate::{ConfigRepository, ConfigSnapshot, ProfileError};
pub use schema::ExecutionConfig;

impl ConfigRepository {
    /// Canonicalize host input before recording it in the session ledger.
    pub fn normalize_execution(
        &self,
        input: &ExecutionConfig,
    ) -> Result<ExecutionConfig, ProfileError> {
        input
            .validate()
            .map_err(|reason| invalid(reason.to_owned()))?;
        let canonical = |path: &str| -> Result<String, ProfileError> {
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
