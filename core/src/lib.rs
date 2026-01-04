//! Core module for delocaliser pipeline orchestration.
//!
//! This module provides the main pipeline coordination and shared utilities.

/// Placeholder for core functionality.
pub fn version() -> &'static str {
    "0.1.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(version(), "0.1.0");
    }
}
