//! Confusables Database
//!
//! This module contains the confusable character mappings used for homoglyph detection.
//! Data sourced from Unicode confusables and common attack patterns.

use std::collections::HashMap;
use lazy_static::lazy_static;

/// A confusable character mapping
#[derive(Debug, Clone)]
pub struct ConfusableEntry {
    /// The confusable character
    pub confusable: char,
    /// The base character it resembles
    pub base: char,
    /// Script/source of the confusable
    pub script: &'static str,
    /// Visual similarity score (0.0 - 1.0)
    pub similarity: f32,
}

lazy_static! {
    /// Main confusables database - maps base char to list of confusables
    pub static ref CONFUSABLES_DB: HashMap<char, Vec<ConfusableEntry>> = {
        let mut map: HashMap<char, Vec<ConfusableEntry>> = HashMap::new();
        
        // === LATIN 'a' CONFUSABLES ===
        map.insert('a', vec![
            ConfusableEntry { confusable: 'а', base: 'a', script: "Cyrillic", similarity: 1.0 },  // U+0430
            ConfusableEntry { confusable: 'α', base: 'a', script: "Greek", similarity: 0.95 },    // U+03B1
            ConfusableEntry { confusable: 'ɑ', base: 'a', script: "IPA", similarity: 0.9 },       // U+0251
            ConfusableEntry { confusable: 'а', base: 'a', script: "Armenian", similarity: 0.9 },  // U+0561
        ]);
        
        // === LATIN 'b' CONFUSABLES ===
        map.insert('b', vec![
            ConfusableEntry { confusable: 'Ь', base: 'b', script: "Cyrillic", similarity: 0.95 },  // U+042C
            ConfusableEntry { confusable: 'β', base: 'b', script: "Greek", similarity: 0.85 },     // U+03B2
        ]);
        
        // === LATIN 'c' CONFUSABLES ===
        map.insert('c', vec![
            ConfusableEntry { confusable: 'с', base: 'c', script: "Cyrillic", similarity: 1.0 },   // U+0441
            ConfusableEntry { confusable: 'ϲ', base: 'c', script: "Greek", similarity: 0.95 },     // U+03F2
            ConfusableEntry { confusable: 'ⅽ', base: 'c', script: "Roman Numeral", similarity: 0.9 }, // U+217D
        ]);
        
        // === LATIN 'd' CONFUSABLES ===
        map.insert('d', vec![
            ConfusableEntry { confusable: 'ԁ', base: 'd', script: "Cyrillic", similarity: 0.95 },  // U+0501
        ]);
        
        // === LATIN 'e' CONFUSABLES ===
        map.insert('e', vec![
            ConfusableEntry { confusable: 'е', base: 'e', script: "Cyrillic", similarity: 1.0 },   // U+0435
            ConfusableEntry { confusable: 'ε', base: 'e', script: "Greek", similarity: 0.95 },     // U+03B5
            ConfusableEntry { confusable: 'ϵ', base: 'e', script: "Greek", similarity: 0.95 },     // U+03F5
            ConfusableEntry { confusable: '℮', base: 'e', script: "Symbol", similarity: 0.85 },    // U+212E
        ]);
        
        // === LATIN 'g' CONFUSABLES ===
        map.insert('g', vec![
            ConfusableEntry { confusable: 'ɡ', base: 'g', script: "IPA", similarity: 0.95 },       // U+0261
        ]);
        
        // === LATIN 'h' CONFUSABLES ===
        map.insert('h', vec![
            ConfusableEntry { confusable: 'һ', base: 'h', script: "Cyrillic", similarity: 0.95 },  // U+04BB
        ]);
        
        // === LATIN 'i' CONFUSABLES ===
        map.insert('i', vec![
            ConfusableEntry { confusable: 'і', base: 'i', script: "Cyrillic", similarity: 1.0 },   // U+0456
            ConfusableEntry { confusable: 'ι', base: 'i', script: "Greek", similarity: 0.9 },      // U+03B9
            ConfusableEntry { confusable: 'ⅈ', base: 'i', script: "Symbol", similarity: 0.85 },    // U+2148
        ]);
        
        // === LATIN 'j' CONFUSABLES ===
        map.insert('j', vec![
            ConfusableEntry { confusable: 'ј', base: 'j', script: "Cyrillic", similarity: 0.95 },  // U+0458
        ]);
        
        // === LATIN 'k' CONFUSABLES ===
        map.insert('k', vec![
            ConfusableEntry { confusable: 'κ', base: 'k', script: "Greek", similarity: 0.95 },     // U+03BA
            ConfusableEntry { confusable: 'k', base: 'k', script: "IPA", similarity: 0.9 },        // U+0138
        ]);
        
        // === LATIN 'l' CONFUSABLES ===
        map.insert('l', vec![
            ConfusableEntry { confusable: 'ӏ', base: 'l', script: "Cyrillic", similarity: 0.95 },  // U+04CF
            ConfusableEntry { confusable: 'ⅼ', base: 'l', script: "Roman Numeral", similarity: 0.9 }, // U+217C
            ConfusableEntry { confusable: '丨', base: 'l', script: "CJK", similarity: 0.85 },      // U+4E28
        ]);
        
        // === LATIN 'm' CONFUSABLES ===
        map.insert('m', vec![
            ConfusableEntry { confusable: 'м', base: 'm', script: "Cyrillic", similarity: 1.0 },   // U+043C
            ConfusableEntry { confusable: 'μ', base: 'm', script: "Greek", similarity: 0.85 },     // U+03BC
        ]);
        
        // === LATIN 'n' CONFUSABLES ===
        map.insert('n', vec![
            ConfusableEntry { confusable: 'ո', base: 'n', script: "Armenian", similarity: 0.95 },  // U+0578
            ConfusableEntry { confusable: 'ռ', base: 'n', script: "Armenian", similarity: 0.85 },  // U+057C
        ]);
        
        // === LATIN 'o' CONFUSABLES ===
        map.insert('o', vec![
            ConfusableEntry { confusable: 'о', base: 'o', script: "Cyrillic", similarity: 1.0 },   // U+043E
            ConfusableEntry { confusable: 'ο', base: 'o', script: "Greek", similarity: 1.0 },      // U+03BF
            ConfusableEntry { confusable: 'օ', base: 'o', script: "Armenian", similarity: 0.95 },  // U+0585
            ConfusableEntry { confusable: '°', base: 'o', script: "Symbol", similarity: 0.8 },     // U+00B0
        ]);
        
        // === LATIN 'p' CONFUSABLES ===
        map.insert('p', vec![
            ConfusableEntry { confusable: 'р', base: 'p', script: "Cyrillic", similarity: 1.0 },   // U+0440
            ConfusableEntry { confusable: 'ρ', base: 'p', script: "Greek", similarity: 0.95 },     // U+03C1
            ConfusableEntry { confusable: 'ϱ', base: 'p', script: "Greek", similarity: 0.9 },      // U+03F1
        ]);
        
        // === LATIN 'r' CONFUSABLES ===
        map.insert('r', vec![
            ConfusableEntry { confusable: 'г', base: 'r', script: "Cyrillic", similarity: 0.9 },   // U+0433
            ConfusableEntry { confusable: 'ʀ', base: 'r', script: "IPA", similarity: 0.85 },       // U+0280
        ]);
        
        // === LATIN 's' CONFUSABLES ===
        map.insert('s', vec![
            ConfusableEntry { confusable: 'ѕ', base: 's', script: "Cyrillic", similarity: 1.0 },   // U+0455
            ConfusableEntry { confusable: 'ϛ', base: 's', script: "Greek", similarity: 0.85 },     // U+03DB
        ]);
        
        // === LATIN 't' CONFUSABLES ===
        map.insert('t', vec![
            ConfusableEntry { confusable: 'т', base: 't', script: "Cyrillic", similarity: 1.0 },   // U+0442
            ConfusableEntry { confusable: 'τ', base: 't', script: "Greek", similarity: 0.9 },      // U+03C4
        ]);
        
        // === LATIN 'u' CONFUSABLES ===
        map.insert('u', vec![
            ConfusableEntry { confusable: 'υ', base: 'u', script: "Greek", similarity: 0.9 },      // U+03C5
            ConfusableEntry { confusable: 'ʋ', base: 'u', script: "IPA", similarity: 0.85 },       // U+028B
        ]);
        
        // === LATIN 'v' CONFUSABLES ===
        map.insert('v', vec![
            ConfusableEntry { confusable: 'ν', base: 'v', script: "Greek", similarity: 0.95 },     // U+03BD
            ConfusableEntry { confusable: 'ѵ', base: 'v', script: "Cyrillic", similarity: 0.85 },  // U+0475
        ]);
        
        // === LATIN 'w' CONFUSABLES ===
        map.insert('w', vec![
            ConfusableEntry { confusable: 'ԝ', base: 'w', script: "Cyrillic", similarity: 0.95 },  // U+051D
            ConfusableEntry { confusable: 'ω', base: 'w', script: "Greek", similarity: 0.8 },      // U+03C9
        ]);
        
        // === LATIN 'x' CONFUSABLES ===
        map.insert('x', vec![
            ConfusableEntry { confusable: 'х', base: 'x', script: "Cyrillic", similarity: 1.0 },   // U+0445
            ConfusableEntry { confusable: 'χ', base: 'x', script: "Greek", similarity: 0.95 },     // U+03C7
        ]);
        
        // === LATIN 'y' CONFUSABLES ===
        map.insert('y', vec![
            ConfusableEntry { confusable: 'у', base: 'y', script: "Cyrillic", similarity: 1.0 },   // U+0443
            ConfusableEntry { confusable: 'γ', base: 'y', script: "Greek", similarity: 0.85 },     // U+03B3
        ]);
        
        // === LATIN 'z' CONFUSABLES ===
        map.insert('z', vec![
            ConfusableEntry { confusable: 'ȥ', base: 'z', script: "IPA", similarity: 0.85 },       // U+0225
        ]);
        
        // === ADDITIONAL COMMON CONFUSABLES ===
        
        // A (uppercase)
        map.insert('A', vec![
            ConfusableEntry { confusable: 'А', base: 'A', script: "Cyrillic", similarity: 1.0 },   // U+0410
            ConfusableEntry { confusable: 'Α', base: 'A', script: "Greek", similarity: 1.0 },      // U+0391
            ConfusableEntry { confusable: 'Ꭺ', base: 'A', script: "Cherokee", similarity: 0.9 },  // U+13AA
        ]);
        
        // B (uppercase)
        map.insert('B', vec![
            ConfusableEntry { confusable: 'В', base: 'B', script: "Cyrillic", similarity: 1.0 },   // U+0412
            ConfusableEntry { confusable: 'Β', base: 'B', script: "Greek", similarity: 1.0 },      // U+0392
        ]);
        
        // C (uppercase)
        map.insert('C', vec![
            ConfusableEntry { confusable: 'С', base: 'C', script: "Cyrillic", similarity: 1.0 },   // U+0421
            ConfusableEntry { confusable: 'Ϲ', base: 'C', script: "Greek", similarity: 0.95 },     // U+03F9
        ]);
        
        // E (uppercase)
        map.insert('E', vec![
            ConfusableEntry { confusable: 'Е', base: 'E', script: "Cyrillic", similarity: 1.0 },   // U+0415
            ConfusableEntry { confusable: 'Ε', base: 'E', script: "Greek", similarity: 1.0 },      // U+0395
        ]);
        
        // H (uppercase)
        map.insert('H', vec![
            ConfusableEntry { confusable: 'Н', base: 'H', script: "Cyrillic", similarity: 1.0 },   // U+041D
            ConfusableEntry { confusable: 'Η', base: 'H', script: "Greek", similarity: 1.0 },      // U+0397
        ]);
        
        // I (uppercase)
        map.insert('I', vec![
            ConfusableEntry { confusable: 'І', base: 'I', script: "Cyrillic", similarity: 1.0 },   // U+0406
            ConfusableEntry { confusable: 'Ι', base: 'I', script: "Greek", similarity: 1.0 },      // U+0399
        ]);
        
        // K (uppercase)
        map.insert('K', vec![
            ConfusableEntry { confusable: 'Κ', base: 'K', script: "Greek", similarity: 1.0 },      // U+039A
        ]);
        
        // M (uppercase)
        map.insert('M', vec![
            ConfusableEntry { confusable: 'М', base: 'M', script: "Cyrillic", similarity: 1.0 },   // U+041C
            ConfusableEntry { confusable: 'Μ', base: 'M', script: "Greek", similarity: 1.0 },      // U+039C
        ]);
        
        // N (uppercase)
        map.insert('N', vec![
            ConfusableEntry { confusable: 'Ν', base: 'N', script: "Greek", similarity: 1.0 },      // U+039D
        ]);
        
        // O (uppercase)
        map.insert('O', vec![
            ConfusableEntry { confusable: 'О', base: 'O', script: "Cyrillic", similarity: 1.0 },   // U+041E
            ConfusableEntry { confusable: 'Ο', base: 'O', script: "Greek", similarity: 1.0 },      // U+039F
        ]);
        
        // P (uppercase)
        map.insert('P', vec![
            ConfusableEntry { confusable: 'Р', base: 'P', script: "Cyrillic", similarity: 1.0 },   // U+0420
            ConfusableEntry { confusable: 'Ρ', base: 'P', script: "Greek", similarity: 1.0 },      // U+03A1
        ]);
        
        // T (uppercase)
        map.insert('T', vec![
            ConfusableEntry { confusable: 'Т', base: 'T', script: "Cyrillic", similarity: 1.0 },   // U+0422
            ConfusableEntry { confusable: 'Τ', base: 'T', script: "Greek", similarity: 1.0 },      // U+03A4
        ]);
        
        // X (uppercase)
        map.insert('X', vec![
            ConfusableEntry { confusable: 'Х', base: 'X', script: "Cyrillic", similarity: 1.0 },   // U+0425
            ConfusableEntry { confusable: 'Χ', base: 'X', script: "Greek", similarity: 1.0 },      // U+03A7
        ]);
        
        // Y (uppercase)
        map.insert('Y', vec![
            ConfusableEntry { confusable: 'Υ', base: 'Y', script: "Greek", similarity: 1.0 },      // U+03A5
        ]);
        
        map
    };

    /// Reverse map: confusable -> base character
    pub static ref REVERSE_CONFUSABLES: HashMap<char, char> = {
        let mut map = HashMap::new();
        for (base, entries) in CONFUSABLES_DB.iter() {
            for entry in entries {
                map.insert(entry.confusable, *base);
            }
        }
        map
    };

    /// Set of all confusable characters for quick lookup
    pub static ref ALL_CONFUSABLES: std::collections::HashSet<char> = {
        REVERSE_CONFUSABLES.keys().copied().collect()
    };
}

/// Get confusables for a base character
pub fn get_confusables(base: char) -> Option<&'static Vec<ConfusableEntry>> {
    CONFUSABLES_DB.get(&base)
}

/// Check if a character is a confusable and return its base
pub fn get_base_char(confusable: char) -> Option<char> {
    REVERSE_CONFUSABLES.get(&confusable).copied()
}

/// Check if a character is in the confusables database
pub fn is_confusable(ch: char) -> bool {
    ALL_CONFUSABLES.contains(&ch)
}

/// Get the script name for a confusable character
pub fn get_confusable_script(ch: char) -> Option<&'static str> {
    REVERSE_CONFUSABLES.get(&ch).and_then(|base| {
        CONFUSABLES_DB.get(base)
            .and_then(|entries| entries.iter().find(|e| e.confusable == ch))
            .map(|e| e.script)
    })
}

/// Get similarity score for a confusable
pub fn get_similarity(confusable: char) -> Option<f32> {
    REVERSE_CONFUSABLES.get(&confusable).and_then(|base| {
        CONFUSABLES_DB.get(base)
            .and_then(|entries| entries.iter().find(|e| e.confusable == confusable))
            .map(|e| e.similarity)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyrillic_a_detection() {
        assert!(is_confusable('а')); // Cyrillic
        assert_eq!(get_base_char('а'), Some('a'));
        assert_eq!(get_confusable_script('а'), Some("Cyrillic"));
    }

    #[test]
    fn test_greek_o_detection() {
        assert!(is_confusable('ο')); // Greek
        assert_eq!(get_base_char('ο'), Some('o'));
        assert_eq!(get_confusable_script('ο'), Some("Greek"));
    }

    #[test]
    fn test_uppercase_confusables() {
        assert!(is_confusable('А')); // Cyrillic A
        assert!(is_confusable('В')); // Cyrillic B
        assert!(is_confusable('С')); // Cyrillic C
        assert_eq!(get_base_char('А'), Some('A'));
    }

    #[test]
    fn test_non_confusable() {
        assert!(!is_confusable('x')); // Regular Latin x
        assert!(!is_confusable('1')); // Digit
        assert!(!is_confusable('!')); // Punctuation
    }

    #[test]
    fn test_similarity_scores() {
        // Cyrillic characters should have high similarity
        assert!(get_similarity('а').unwrap_or(0.0) >= 0.9);
        assert!(get_similarity('о').unwrap_or(0.0) >= 0.9);
    }

    #[test]
    fn test_database_size() {
        // Should have at least 50 confusable entries
        assert!(ALL_CONFUSABLES.len() >= 50);
    }
}
