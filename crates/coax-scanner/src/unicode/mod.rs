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
pub mod findings;
pub mod ranges;
pub mod confusables;
pub mod detectors;
pub mod scanner;

pub use config::{
    UnicodeConfig,
    DetectorConfig,
    AllowlistConfig,
    PerformanceConfig,
    SensitivityLevel,
};

pub use findings::{
    UnicodeFinding,
    UnicodeCategory,
    Severity,
    SourceLocation,
    UnicodeScanStats,
};

pub use ranges::{
    UnicodeRange,
    INVISIBLE_RANGES,
    CRITICAL_RANGES,
    is_in_invisible_range,
    is_in_critical_range,
    is_variation_selector,
    get_bidi_name,
    get_zero_width_name,
};

pub use confusables::data::{
    ConfusableEntry,
    CONFUSABLES_DB,
    REVERSE_CONFUSABLES,
    ALL_CONFUSABLES,
    get_confusables,
    get_base_char,
    is_confusable,
    get_confusable_script,
    get_similarity,
};

pub use detectors::{
    InvisibleCharDetector,
    HomoglyphDetector,
    BidiDetector,
    GlasswormDetector,
    UnicodeTagDetector,
    GlasswormIndicator,
    ConfusableMatch,
    UnicodeDetector,
};

pub use scanner::{
    UnicodeScanner,
    ScanSessionStats,
};

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
            "test", 1, 1, 0, 'a',
            UnicodeCategory::Unknown, Severity::Low,
            "test", "test"
        );
    }
}
