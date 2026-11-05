//! Rust bindings for the ALPM library
//!
//! SigmaOS component inspired by Arch Linux's `alpm-rs` repository.
//! Designed for component isolation, loose coupling, and secure defaults.

use std::fmt;

/// Errors produced by this component. Fail-fast: errors are returned, not panicked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sigmaos_alpm_rsError {
    /// The requested operation is not valid in the current state.
    InvalidState,
    /// Input failed validation.
    InvalidInput(String),
}

impl fmt::Display for Sigmaos_alpm_rsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidState => write!(f, "invalid state"),
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
        }
    }
}

impl std::error::Error for Sigmaos_alpm_rsError {}

/// Core component type. Holds configuration; performs no I/O by default.
#[derive(Debug, Clone, Default)]
pub struct Sigmaos_alpm_rs {
    name: String,
}

impl Sigmaos_alpm_rs {
    /// Create a new component instance. Fails fast on empty names.
    pub fn new(name: impl Into<String>) -> Result<Self, Sigmaos_alpm_rsError> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(Sigmaos_alpm_rsError::InvalidInput("name must not be empty".into()));
        }
        Ok(Self { name })
    }

    /// Component identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Perform the component's primary action. Returns a deterministic result.
    pub fn run(&self) -> Result<String, Sigmaos_alpm_rsError> {
        if self.name.is_empty() {
            return Err(Sigmaos_alpm_rsError::InvalidState);
        }
        Ok(format!("{}: ok", self.name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_rejects_empty_name() {
        // Arrange / Act / Assert
        assert!(matches!(Sigmaos_alpm_rs::new("  "), Err(Sigmaos_alpm_rsError::InvalidInput(_))));
    }

    #[test]
    fn run_returns_deterministic_result() {
        let c = Sigmaos_alpm_rs::new("test").unwrap();
        assert_eq!(c.run().unwrap(), "test: ok");
    }

    #[test]
    fn name_is_accessible() {
        let c = Sigmaos_alpm_rs::new("abc").unwrap();
        assert_eq!(c.name(), "abc");
    }
}
