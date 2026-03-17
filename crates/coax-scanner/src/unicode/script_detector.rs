//! Script Detector Module
//!
//! Detects Unicode script mixing in identifiers to identify potential homoglyph attacks
//! while allowing legitimate non-Latin identifiers (i18n, mathematical notation).
//!
//! ## Key Insight
//!
//! Pure non-Latin identifiers (e.g., Greek "μήνυμα", Cyrillic "сообщение") are legitimate
//! for i18n purposes. However, MIXED scripts within a single identifier (e.g., "variαble"
//! where α is Greek) indicates a potential homoglyph attack.

use std::collections::HashSet;
use unicode_script::{Script, UnicodeScript};

/// Detect which Unicode script a character belongs to
pub fn get_script(ch: char) -> Script {
    ch.script()
}

/// Check if an identifier contains mixed scripts (potential homoglyph attack)
///
/// Returns true if:
/// - The identifier contains both Latin (ASCII) AND non-Latin scripts
/// - This mixing is deceptive and indicates a potential attack
pub fn has_mixed_scripts(identifier: &str) -> bool {
    let mut non_latin_scripts = HashSet::new();
    let mut has_latin = false;

    for ch in identifier.chars() {
        // Track ASCII Latin characters (but NOT underscore - it's common across scripts)
        if ch.is_ascii_alphabetic() {
            has_latin = true;
            continue;
        }

        // Skip common characters (underscore, digits, etc.)
        if ch == '_' || ch.is_ascii_digit() {
            continue;
        }

        let script = get_script(ch);

        // Only count non-Latin scripts
        if script != Script::Latin && script != Script::Common && script != Script::Inherited {
            non_latin_scripts.insert(script);
        }
    }

    // Mixed scripts = has Latin + 1+ non-Latin scripts
    has_latin && !non_latin_scripts.is_empty()
}

/// Check if identifier is pure non-Latin script (legitimate i18n)
///
/// Returns true if:
/// - The identifier contains non-Latin scripts
/// - The identifier does NOT contain any Latin ASCII characters
/// - This is legitimate for i18n, math notation, etc.
pub fn is_pure_non_latin(identifier: &str) -> bool {
    // Check for actual Latin letters (not underscore or digits)
    let has_latin = identifier.chars().any(|c| c.is_ascii_alphabetic());
    let has_non_latin = identifier.chars().any(|c| {
        let script = get_script(c);
        script != Script::Latin && script != Script::Common && script != Script::Inherited
    });

    // Pure non-Latin = has non-Latin but NO Latin letters
    has_non_latin && !has_latin
}

/// Check if identifier is pure Latin (no flags needed)
pub fn is_pure_latin(identifier: &str) -> bool {
    identifier
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Get all scripts present in an identifier
pub fn get_scripts_in_identifier(identifier: &str) -> Vec<Script> {
    let mut scripts = HashSet::new();

    for ch in identifier.chars() {
        let script = get_script(ch);
        if script != Script::Common && script != Script::Inherited {
            scripts.insert(script);
        }
    }

    scripts.into_iter().collect()
}

/// Check if a character is from a high-risk script (commonly used in attacks)
pub fn is_high_risk_script(ch: char) -> bool {
    let script = get_script(ch);
    // Cyrillic and Greek are most commonly used in homoglyph attacks
    script == Script::Cyrillic || script == Script::Greek
}

/// Get the script name as a string for reporting
pub fn script_to_string(script: Script) -> &'static str {
    match script {
        Script::Latin => "Latin",
        Script::Greek => "Greek",
        Script::Cyrillic => "Cyrillic",
        Script::Arabic => "Arabic",
        Script::Hebrew => "Hebrew",
        Script::Han => "Han (Chinese)",
        Script::Hiragana => "Hiragana",
        Script::Katakana => "Katakana",
        Script::Hangul => "Hangul (Korean)",
        Script::Thai => "Thai",
        Script::Devanagari => "Devanagari",
        Script::Common => "Common",
        Script::Inherited => "Inherited",
        _ => "Other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_greek_not_flagged() {
        // Pure Greek identifiers should NOT be flagged as mixed
        assert!(is_pure_non_latin("μήνυμα"));
        assert!(is_pure_non_latin("αβγ"));
        assert!(is_pure_non_latin("υπολογισμός"));
        assert!(!has_mixed_scripts("μήνυμα"));
        assert!(!has_mixed_scripts("αβγ"));
    }

    #[test]
    fn test_pure_cyrillic_not_flagged() {
        // Pure Cyrillic identifiers should NOT be flagged as mixed
        assert!(is_pure_non_latin("сообщение"));
        assert!(is_pure_non_latin("абв"));
        assert!(!has_mixed_scripts("сообщение"));
    }

    #[test]
    fn test_mixed_script_flagged() {
        // Latin + Greek mixing (deceptive)
        assert!(has_mixed_scripts("variαble")); // α is Greek
        assert!(has_mixed_scripts("pαypal")); // α is Greek
        assert!(!is_pure_non_latin("variαble"));

        // Latin + Cyrillic mixing (more deceptive)
        assert!(has_mixed_scripts("pаypal")); // а is Cyrillic
        assert!(has_mixed_scripts("vаriable")); // а is Cyrillic
        assert!(has_mixed_scripts("fаke")); // а is Cyrillic
    }

    #[test]
    fn test_pure_latin_not_flagged() {
        assert!(!has_mixed_scripts("variable"));
        assert!(!has_mixed_scripts("username"));
        assert!(!has_mixed_scripts("test_variable_123"));
        assert!(!is_pure_non_latin("variable"));
    }

    #[test]
    fn test_mathematical_greek_not_flagged() {
        // Pure Greek math variables should NOT be flagged
        assert!(is_pure_non_latin("θ"));
        assert!(is_pure_non_latin("φ"));
        assert!(is_pure_non_latin("Δ"));
        assert!(!has_mixed_scripts("θ"));
        assert!(!has_mixed_scripts("Δ"));
    }

    #[test]
    fn test_script_detection() {
        assert_eq!(get_script('a'), Script::Latin);
        assert_eq!(get_script('α'), Script::Greek); // Greek alpha
        assert_eq!(get_script('а'), Script::Cyrillic); // Cyrillic a
        assert_eq!(get_script('5'), Script::Common);
    }

    #[test]
    fn test_get_scripts_in_identifier() {
        let scripts = get_scripts_in_identifier("variαble");
        assert!(scripts.contains(&Script::Latin));
        assert!(scripts.contains(&Script::Greek));

        let scripts2 = get_scripts_in_identifier("μήνυμα");
        assert!(scripts2.contains(&Script::Greek));
        assert!(!scripts2.contains(&Script::Latin));
    }

    #[test]
    fn test_high_risk_scripts() {
        assert!(is_high_risk_script('α')); // Greek
        assert!(is_high_risk_script('а')); // Cyrillic
        assert!(!is_high_risk_script('a')); // Latin
        assert!(!is_high_risk_script('中')); // Han (not high-risk for homoglyphs)
    }
}

/// Extract JavaScript/TypeScript/Python identifiers from a line
pub fn extract_identifiers(line: &str) -> Vec<&str> {
    lazy_static::lazy_static! {
        static ref IDENTIFIER_PATTERN: regex::Regex =
            regex::Regex::new(r"\b[\p{L}_$][\p{L}\p{N}_$]*\b").unwrap();
    }

    IDENTIFIER_PATTERN
        .find_iter(line)
        .map(|m| m.as_str())
        .collect()
}

/// Find the identifier containing a specific character position
pub fn find_identifier_at_position<'a>(
    line: &'a str,
    char_pos: usize,
    identifiers: &[&'a str],
) -> Option<&'a str> {
    let byte_pos = line.char_indices().nth(char_pos)?.0;

    for id in identifiers {
        if let Some(start) = line.find(id) {
            let end = start + id.len();
            if byte_pos >= start && byte_pos < end {
                return Some(id);
            }
        }
    }

    None
}
