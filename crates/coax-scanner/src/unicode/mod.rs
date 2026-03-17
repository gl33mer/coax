//! Unicode Attack Detection Module
//!
//! This module provides comprehensive Unicode attack detection for the Coax security scanner.
//!
//! ## Features
//!
//! - **Invisible Character Detection**: Detects zero-width characters, variation selectors,
//!   and other invisible Unicode used in Glassworm-style attacks
//! - **Homoglyph Detection**: Identifies confusable characters from Cyrillic, Greek, and
//!   other scripts that could be used for spoofing
//! - **Bidirectional Override Detection**: Finds dangerous bidi control characters that
//!   can reverse text display
//! - **Glassworm Pattern Detection**: Specialized detection for Glassworm attack patterns
//!   including decoder functions and eval usage
//! - **Unicode Tag Detection**: Identifies tag characters used for metadata injection
//!
//! ## Architecture
//!
//! ```text
//! Input → UnicodeScanner → [Detectors] → Findings → Output
//!                          ├─ Invisible
//!                          ├─ Homoglyph
//!                          ├─ Bidi
//!                          ├─ Glassworm
//!                          └─ Tags
//! ```
//!
//! ## Example Usage
//!
//! ```rust
//! use coax_scanner::unicode::{UnicodeScanner, UnicodeConfig};
//!
//! // Create scanner with default config
//! let scanner = UnicodeScanner::with_default_config();
//!
//! // Scan content
//! let content = "const secret\u{FE00}Key = 'value';";
//! let findings = scanner.scan(content, "test.js");
//!
//! // Process findings
//! for finding in findings {
//!     println!("Found: {}", finding.description);
//! }
//! ```
//!
//! ## Performance
//!
//! - Time complexity: O(n) where n = number of characters
//! - Space complexity: O(1) beyond input storage
//! - Confusables lookup: O(1) using HashMap
//!
//! ## Configuration
//!
//! See [`UnicodeConfig`] for configuration options including:
//! - Sensitivity levels (low, medium, high, critical)
//! - Per-detector enable/disable
//! - File include/exclude patterns
//! - Allowlist for legitimate i18n usage

pub mod config;
pub mod confusables;
pub mod detectors;
pub mod findings;
pub mod ranges;
pub mod scanner;
pub mod script_detector;

pub use config::{
    AllowlistConfig, DetectorConfig, PerformanceConfig, SensitivityLevel, UnicodeConfig,
};

pub use findings::{Severity, SourceLocation, UnicodeCategory, UnicodeFinding, UnicodeScanStats};

pub use ranges::{
    get_bidi_name, get_zero_width_name, is_in_critical_range, is_in_invisible_range,
    is_variation_selector, UnicodeRange, CRITICAL_RANGES, INVISIBLE_RANGES,
};

pub use confusables::data::{
    get_base_char, get_confusable_script, get_confusables, get_similarity, is_confusable,
    ConfusableEntry, ALL_CONFUSABLES, CONFUSABLES_DB, REVERSE_CONFUSABLES,
};

// Script detection utilities
pub use script_detector::{
    get_script, get_scripts_in_identifier, has_mixed_scripts, is_high_risk_script, is_pure_latin,
    is_pure_non_latin, script_to_string,
};

pub use detectors::{
    BidiDetector, ConfusableMatch, GlasswormDetector, GlasswormIndicator, HomoglyphDetector,
    InvisibleCharDetector, UnicodeDetector, UnicodeTagDetector,
};

pub use scanner::{ScanSessionStats, UnicodeScanner};

/// Module version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all major types are exported
        let _config = UnicodeConfig::default();
        let _scanner = UnicodeScanner::with_default_config();
        let _finding = UnicodeFinding::new(
            "test",
            1,
            1,
            0,
            'a',
            UnicodeCategory::Unknown,
            Severity::Low,
            "test",
            "test",
        );
    }
}
