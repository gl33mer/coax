//! Unicode Range Definitions
//!
//! This module defines the Unicode ranges used for attack detection.

use lazy_static::lazy_static;

/// A named Unicode range
#[derive(Debug, Clone)]
pub struct UnicodeRange {
    pub start: u32,
    pub end: u32,
    pub name: &'static str,
    pub description: &'static str,
}

impl UnicodeRange {
    pub const fn new(start: u32, end: u32, name: &'static str, description: &'static str) -> Self {
        Self { start, end, name, description }
    }

    pub fn contains(&self, code_point: u32) -> bool {
        code_point >= self.start && code_point <= self.end
    }
}

// Variation Selectors (Glassworm primary range)
pub const VARIATION_SELECTORS: UnicodeRange = UnicodeRange::new(
    0xFE00, 0xFE0F,
    "Variation Selectors",
    "Used by Glassworm to hide payloads"
);

// Variation Selectors Supplement
pub const VARIATION_SELECTORS_SUPPLEMENT: UnicodeRange = UnicodeRange::new(
    0xE0100, 0xE01EF,
    "Variation Selectors Supplement",
    "Extended variation selectors"
);

// Zero-width characters
pub const ZERO_WIDTH_SPACE: UnicodeRange = UnicodeRange::new(
    0x200B, 0x200F,
    "Zero-Width Characters",
    "Invisible characters used for injection attacks"
);

// Word joiner and invisible operators
pub const INVISIBLE_OPERATORS: UnicodeRange = UnicodeRange::new(
    0x2060, 0x206F,
    "Invisible Operators",
    "Hidden formatting characters"
);

// Bidirectional overrides
pub const BIDIRECTIONAL_FORMATTING: UnicodeRange = UnicodeRange::new(
    0x202A, 0x202E,
    "Bidirectional Formatting",
    "Text direction override characters"
);

// Isolation characters
pub const ISOLATION_CHARACTERS: UnicodeRange = UnicodeRange::new(
    0x2066, 0x2069,
    "Isolation Characters",
    "Unicode isolation formatting"
);

// Unicode tags
pub const UNICODE_TAGS: UnicodeRange = UnicodeRange::new(
    0xE0000, 0xE007F,
    "Unicode Tags",
    "Tag characters for metadata injection"
);

// Private Use Area (can be used for custom attacks)
pub const PRIVATE_USE_AREA_A: UnicodeRange = UnicodeRange::new(
    0xE000, 0xF8FF,
    "Private Use Area A",
    "Custom/private characters"
);

// Specials
pub const SPECIALS: UnicodeRange = UnicodeRange::new(
    0xFFF0, 0xFFFF,
    "Specials",
    "Special purpose characters"
);

lazy_static! {
    /// All invisible character ranges
    pub static ref INVISIBLE_RANGES: Vec<UnicodeRange> = vec![
        VARIATION_SELECTORS,
        VARIATION_SELECTORS_SUPPLEMENT,
        ZERO_WIDTH_SPACE,
        INVISIBLE_OPERATORS,
        BIDIRECTIONAL_FORMATTING,
        ISOLATION_CHARACTERS,
        UNICODE_TAGS,
        SPECIALS,
    ];

    /// Critical ranges (always scan regardless of sensitivity)
    pub static ref CRITICAL_RANGES: Vec<UnicodeRange> = vec![
        VARIATION_SELECTORS,
        VARIATION_SELECTORS_SUPPLEMENT,
        ZERO_WIDTH_SPACE,
        BIDIRECTIONAL_FORMATTING,
    ];

    /// Bidirectional control characters
    pub static ref BIDI_CHARS: Vec<(u32, &'static str)> = vec![
        (0x202A, "LRE"),  // Left-to-Right Embedding
        (0x202B, "RLE"),  // Right-to-Left Embedding
        (0x202C, "PDF"),  // Pop Directional Formatting
        (0x202D, "LRO"),  // Left-to-Right Override
        (0x202E, "RLO"),  // Right-to-Left Override (MOST DANGEROUS)
        (0x2066, "LRI"),  // Left-to-Right Isolate
        (0x2067, "RLI"),  // Right-to-Left Isolate
        (0x2068, "FSI"),  // First Strong Isolate
        (0x2069, "PDI"),  // Pop Directional Isolate
        (0x200E, "LRM"),  // Left-to-Right Mark
        (0x200F, "RLM"),  // Right-to-Left Mark
        (0x061C, "ALM"),  // Arabic Letter Mark
    ];

    /// Zero-width characters
    pub static ref ZERO_WIDTH_CHARS: Vec<(u32, &'static str)> = vec![
        (0x200B, "ZWSP"),  // Zero Width Space
        (0x200C, "ZWNJ"),  // Zero Width Non-Joiner
        (0x200D, "ZWJ"),   // Zero Width Joiner
        (0x2060, "WJ"),    // Word Joiner
        (0xFEFF, "BOM"),   // Byte Order Mark (when not at start)
    ];

    /// Variation selectors
    pub static ref VARIATION_SELECTOR_CHARS: Vec<(u32, &'static str)> = {
        let mut chars = Vec::new();
        // Variation Selectors-1 to 16
        for i in 0xFE00..=0xFE0F {
            chars.push((i, "VS"));
        }
        // Variation Selectors Supplement
        for i in 0xE0100..=0xE01EF {
            chars.push((i, "VSS"));
        }
        chars
    };
}

/// Check if a code point is in any invisible range
pub fn is_in_invisible_range(code_point: u32) -> bool {
    INVISIBLE_RANGES.iter().any(|r| r.contains(code_point))
}

/// Check if a code point is in a critical range
pub fn is_in_critical_range(code_point: u32) -> bool {
    CRITICAL_RANGES.iter().any(|r| r.contains(code_point))
}

/// Get the name of a bidirectional character
pub fn get_bidi_name(code_point: u32) -> Option<&'static str> {
    BIDI_CHARS.iter()
        .find(|(cp, _)| *cp == code_point)
        .map(|(_, name)| *name)
}

/// Get the name of a zero-width character
pub fn get_zero_width_name(code_point: u32) -> Option<&'static str> {
    ZERO_WIDTH_CHARS.iter()
        .find(|(cp, _)| *cp == code_point)
        .map(|(_, name)| *name)
}

/// Check if a code point is a variation selector
pub fn is_variation_selector(code_point: u32) -> bool {
    VARIATION_SELECTOR_CHARS.iter().any(|(cp, _)| *cp == code_point)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variation_selector_range() {
        assert!(VARIATION_SELECTORS.contains(0xFE00));
        assert!(VARIATION_SELECTORS.contains(0xFE0F));
        assert!(!VARIATION_SELECTORS.contains(0xFE10));
    }

    #[test]
    fn test_is_in_invisible_range() {
        assert!(is_in_invisible_range(0xFE00)); // Variation selector
        assert!(is_in_invisible_range(0x200B)); // Zero-width space
        assert!(is_in_invisible_range(0x202E)); // RLO
        assert!(!is_in_invisible_range(0x0041)); // Latin 'A'
    }

    #[test]
    fn test_is_in_critical_range() {
        assert!(is_in_critical_range(0xFE00));
        assert!(is_in_critical_range(0x200B));
        assert!(is_in_critical_range(0x202E));
        assert!(!is_in_critical_range(0xE0000)); // Tags (not critical)
    }

    #[test]
    fn test_bidi_names() {
        assert_eq!(get_bidi_name(0x202E), Some("RLO"));
        assert_eq!(get_bidi_name(0x202A), Some("LRE"));
        assert_eq!(get_bidi_name(0x0041), None);
    }

    #[test]
    fn test_variation_selector_detection() {
        assert!(is_variation_selector(0xFE00));
        assert!(is_variation_selector(0xFE0F));
        assert!(is_variation_selector(0xE0100));
        assert!(!is_variation_selector(0x200B));
    }
}
