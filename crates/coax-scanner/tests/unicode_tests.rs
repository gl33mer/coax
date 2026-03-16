//! Unicode Detection Integration Tests
//!
//! Comprehensive test suite for Unicode attack detection.

use coax_scanner::unicode::{
    UnicodeScanner, UnicodeConfig, UnicodeFinding, UnicodeCategory,
    SensitivityLevel,
};

/// Test Glassworm pattern detection
#[test]
fn test_glassworm_detection() {
    let scanner = UnicodeScanner::with_default_config();
    let content = r#"
        const hidden = "secret";
        const codes = hidden.split('').map(c => c.codePointAt(0));
        const filtered = codes.filter(c => c !== null);
        eval(String.fromCharCode(...codes));
    "#;
    let findings = scanner.scan(content, "test.js");
    
    assert!(!findings.is_empty());
    assert!(findings.iter().any(|f| f.category == UnicodeCategory::GlasswormPattern));
}

/// Test no false positives on legitimate Unicode
#[test]
fn test_no_false_positives_on_legitimate_unicode() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Chinese comments should not be flagged
    let content = r#"
        # 用户认证模块
        def authenticate(username, password):
            return True
    "#;
    let findings = scanner.scan(content, "test.py");
    
    // Should have 0 findings for legitimate i18n content
    // Note: This depends on config - with default config, only attacks are flagged
    assert_eq!(findings.len(), 0, "Legitimate Unicode should not be flagged");
}

/// Test homoglyph detection accuracy
#[test]

/// Test homoglyph detection accuracy - mixed script identifiers
#[test]
fn test_homoglyph_detection_accuracy() {
    let scanner = UnicodeScanner::with_default_config();

    // Test mixed script identifiers (Latin + non-Latin = deceptive)
    let test_cases = vec![
        ("pаssword", "Cyrillic"),  // Cyrillic а in Latin word
        ("lοgin", "Greek"),        // Greek ο in Latin word
        ("usеr", "Cyrillic"),      // Cyrillic е in Latin word
    ];

    for (identifier, script) in test_cases {
        let content = format!("const {} = 'test';", identifier);
        let findings = scanner.scan(&content, "test.js");

        assert!(
            findings.iter().any(|f| f.category == UnicodeCategory::Homoglyph),
            "Should detect mixed script in '{}' ({} script)", identifier, script
        );
    }
}

fn test_variation_selector_detection() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Variation selector in code
    let content = "const secret\u{FE00}Key = 'value';";
    let findings = scanner.scan(content, "test.js");
    
    assert!(!findings.is_empty());
    assert!(findings.iter().any(|f| {
        f.category == UnicodeCategory::InvisibleCharacter && f.code_point == 0xFE00
    }));
}

/// Test zero-width character detection
#[test]
fn test_zero_width_detection() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Zero-width space
    let content = "const pass\u{200B}word = 'secret';";
    let findings = scanner.scan(content, "test.js");
    
    assert!(!findings.is_empty());
    assert!(findings.iter().any(|f| {
        f.category == UnicodeCategory::InvisibleCharacter && f.code_point == 0x200B
    }));
}

/// Test RLO bidirectional override detection
#[test]
fn test_rlo_bidi_detection() {
    let scanner = UnicodeScanner::with_default_config();
    
    // RLO - most dangerous bidi char
    let content = "const file = \"test\u{202E}exe\";";
    let findings = scanner.scan(content, "test.js");
    
    assert!(!findings.is_empty());
    assert!(findings.iter().any(|f| {
        f.category == UnicodeCategory::BidirectionalOverride && f.code_point == 0x202E
    }));
}

/// Test all bidi characters
#[test]
fn test_all_bidi_characters() {
    let scanner = UnicodeScanner::with_default_config();
    
    let bidi_chars = vec![
        (0x202A, "LRE"),
        (0x202B, "RLE"),
        (0x202C, "PDF"),
        (0x202D, "LRO"),
        (0x202E, "RLO"),
    ];
    
    for (code_point, name) in bidi_chars {
        let ch = char::from_u32(code_point).unwrap();
        let content = format!("const x = \"test{}\";", ch);
        let findings = scanner.scan(&content, "test.js");
        
        assert!(
            findings.iter().any(|f| f.category == UnicodeCategory::BidirectionalOverride),
            "Should detect {} (U+{:04X})", name, code_point
        );
    }
}

/// Test Unicode tag detection
#[test]
fn test_unicode_tag_detection() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Language tag
    let content = "const text = \"hello\u{E0001}world\";";
    let findings = scanner.scan(content, "test.js");
    
    assert!(!findings.is_empty());
    assert!(findings.iter().any(|f| {
        f.category == UnicodeCategory::UnicodeTag && f.code_point == 0xE0001
    }));
}

/// Test clean content produces no findings
#[test]
fn test_clean_content_no_findings() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        const normal = 'hello world';
        let x = 42;
        console.log(normal);
    "#;
    let findings = scanner.scan(content, "test.js");
    
    assert!(findings.is_empty(), "Clean content should have no findings");
}

/// Test sensitivity levels
#[test]
fn test_sensitivity_levels() {
    // Low sensitivity
    let config_low = UnicodeConfig {
        sensitivity: SensitivityLevel::Low,
        ..Default::default()
    };
    let scanner_low = UnicodeScanner::new(config_low);
    
    // Critical sensitivity
    let config_critical = UnicodeConfig {
        sensitivity: SensitivityLevel::Critical,
        ..Default::default()
    };
    let scanner_critical = UnicodeScanner::new(config_critical);
    
    let content = "const secret\u{FE00}Key = 'value';";
    
    let findings_low = scanner_low.scan(content, "test.js");
    let findings_critical = scanner_critical.scan(content, "test.js");
    
    // Critical should catch at least as much as low
    assert!(findings_critical.len() >= findings_low.len());
}

/// Test detector enable/disable
#[test]
fn test_detector_enable_disable() {
    // Disable homoglyph detector
    let mut config = UnicodeConfig::default();
    config.detectors.homoglyphs = false;
    let scanner = UnicodeScanner::new(config);
    
    let content = "const pаssword = 'secret';"; // Cyrillic 'а'
    let findings = scanner.scan(content, "test.js");
    
    // Should not have homoglyph findings
    assert!(!findings.iter().any(|f| f.category == UnicodeCategory::Homoglyph));
}

/// Test has_invisible_chars utility
#[test]
fn test_has_invisible_chars() {
    assert!(UnicodeScanner::has_invisible_chars("hello\u{FE00}world"));
    assert!(UnicodeScanner::has_invisible_chars("test\u{200B}"));
    assert!(UnicodeScanner::has_invisible_chars("file\u{202E}exe"));
    assert!(!UnicodeScanner::has_invisible_chars("normal text"));
}

/// Test has_confusables utility
#[test]
fn test_has_confusables() {
    assert!(UnicodeScanner::has_confusables("pаssword")); // Cyrillic 'а'
    assert!(UnicodeScanner::has_confusables("lοgin")); // Greek 'ο'
    assert!(!UnicodeScanner::has_confusables("password")); // All ASCII
}

/// Test finding deduplication
#[test]
fn test_finding_deduplication() {
    let finding1 = UnicodeFinding::new(
        "test.js", 1, 5, 0xFE00, '\u{FE00}',
        UnicodeCategory::InvisibleCharacter, coax_scanner::unicode::Severity::Critical,
        "test", "fix"
    );
    let finding2 = finding1.clone(); // Duplicate
    let finding3 = UnicodeFinding::new(
        "test.js", 2, 10, 0xFE01, '\u{FE01}',
        UnicodeCategory::InvisibleCharacter, coax_scanner::unicode::Severity::Critical,
        "test", "fix"
    );

    let findings = vec![finding1.clone(), finding2, finding3];
    let deduped = UnicodeScanner::deduplicate_findings(findings);

    assert_eq!(deduped.len(), 2);
}

/// Test i18n configuration
#[test]
fn test_i18n_config() {
    let scanner = UnicodeScanner::for_i18n_project();
    assert_eq!(scanner.get_config().sensitivity.as_str(), "medium");
}

/// Test high security configuration
#[test]
fn test_high_security_config() {
    let scanner = UnicodeScanner::for_high_security();
    assert_eq!(scanner.get_config().sensitivity.as_str(), "critical");
}

/// Test list detectors
#[test]
fn test_list_detectors() {
    let detectors = UnicodeScanner::list_detectors();
    assert_eq!(detectors.len(), 5);
    assert!(detectors.contains(&"invisible_char"));
    assert!(detectors.contains(&"homoglyph"));
    assert!(detectors.contains(&"bidi"));
    assert!(detectors.contains(&"glassworm"));
    assert!(detectors.contains(&"unicode_tag"));
}

/// Test performance on larger content
#[test]
fn test_performance_large_content() {
    let scanner = UnicodeScanner::with_default_config();

    // Generate 10K lines of content
    let mut content = String::new();
    for i in 0..10000 {
        content.push_str(&format!("const line_{} = 'value {}';\n", i, i));
    }

    let start = std::time::Instant::now();
    let findings = scanner.scan(&content, "large.js");
    let elapsed = start.elapsed();

    // Should complete in <500ms for 10K lines in debug mode
    // Release mode should be <100ms
    assert!(
        elapsed < std::time::Duration::from_millis(1000),
        "Scan took {:?}, should be <1s in debug mode", elapsed
    );

    // Clean content should have no findings
    assert!(findings.is_empty());
}

/// Test combined attack patterns
#[test]
fn test_combined_attack_patterns() {
    let scanner = UnicodeScanner::with_default_config();

    // Test homoglyph attack
    let homoglyph_content = "const pаssword = 'secret';";  // Cyrillic а
    let homoglyph_findings = scanner.scan(homoglyph_content, "test.js");
    assert!(homoglyph_findings.iter().any(|f| f.category == UnicodeCategory::Homoglyph),
        "Should detect homoglyph attack");

    // Test bidi attack
    let bidi_content = "const file = \"test\u{202E}exe\";";  // RLO
    let bidi_findings = scanner.scan(bidi_content, "test.js");
    assert!(bidi_findings.iter().any(|f| f.category == UnicodeCategory::BidirectionalOverride),
        "Should detect bidirectional override");

    // Test variation selector
    let vs_content = "const hidden\u{FE00}Key = 'value';";
    let vs_findings = scanner.scan(vs_content, "test.js");
    assert!(vs_findings.iter().any(|f| f.category == UnicodeCategory::InvisibleCharacter),
        "Should detect variation selector");
}

/// v0.7.5 Tests - Script Mixing Detection Fix

/// Test pure Greek identifiers are NOT flagged (v0.7.5 fix)
#[test]
fn test_pure_greek_identifiers_not_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        const μήνυμα = "hello";
        const α = 1;
        const β = 2;
        const γ = α + β;
        const θ = Math.PI / 2;
        const φ = (1 + Math.sqrt(5)) / 2;
        const Δ = b * b - 4 * a * c;
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert_eq!(homoglyph_findings.len(), 0,
        "Pure Greek identifiers should not be flagged as homoglyph attacks");
}

/// Test pure Cyrillic identifiers are NOT flagged (v0.7.5 fix)
#[test]
fn test_pure_cyrillic_identifiers_not_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        const сообщение = "hello";
        const абв = 123;
        const пользователь = "user";
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert_eq!(homoglyph_findings.len(), 0,
        "Pure Cyrillic identifiers should not be flagged as homoglyph attacks");
}

/// Test mixed script identifiers ARE flagged (v0.7.5 fix)
#[test]
fn test_mixed_script_identifiers_are_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Latin + Greek mixing
    let content = r#"
        const variαble = "attack";
        const pαypal = "fake";
        const vаriable = "attack2";
        const pаypal = "attack3";
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert!(homoglyph_findings.len() >= 4,
        "Mixed script identifiers should be flagged (got {} findings)", homoglyph_findings.len());
}

/// Test Greek comments are NOT flagged (v0.7.5 fix)
#[test]
fn test_greek_comments_not_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        // ελληνικά σχόλια - Greek comments
        // comment with α beta γ
        /* More Greek: μήνυμα, αβγ */
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert_eq!(homoglyph_findings.len(), 0,
        "Comments should not be flagged");
}

/// Test Cyrillic comments are NOT flagged (v0.7.5 fix)
#[test]
fn test_cyrillic_comments_not_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        // русские комментарии
        // переменная а не видна
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert_eq!(homoglyph_findings.len(), 0,
        "Cyrillic comments should not be flagged");
}

/// Test mathematical notation is NOT flagged (v0.7.5 fix)
#[test]
fn test_mathematical_greek_not_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        const θ = Math.PI / 2;
        const φ = (1 + Math.sqrt(5)) / 2;  // golden ratio
        const Δ = b * b - 4 * a * c;  // discriminant
        const Σ = sum(values);  // summation
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert_eq!(homoglyph_findings.len(), 0,
        "Mathematical Greek letters should not be flagged");
}

/// Test that real attacks in code are still detected (v0.7.5 regression)
#[test]
fn test_mixed_script_attack_regression() {
    let scanner = UnicodeScanner::with_default_config();
    
    let content = r#"
        const pаssword = "secret";  // Cyrillic 'а' in Latin word
        const lοgin = "user";       // Greek 'ο' in Latin word
        const usеr = "test";        // Cyrillic 'е' in Latin word
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();

    assert!(homoglyph_findings.len() >= 3,
        "Mixed script attacks should still be detected (got {} findings)", homoglyph_findings.len());
}
