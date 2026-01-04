//! UI module for command-line and desktop interfaces.
//!
//! This module provides the CLI and desktop GUI for delocaliser.

/// Placeholder for UI functionality.
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
