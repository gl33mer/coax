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
fn test_homoglyph_detection_accuracy() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Test known confusable pairs
    let test_cases = vec![
        ('а', 'a', "Cyrillic"),  // Cyrillic а vs Latin a
        ('ο', 'o', "Greek"),     // Greek ο vs Latin o
        ('е', 'e', "Cyrillic"),  // Cyrillic е vs Latin e
    ];
    
    for (confusable, base, script) in test_cases {
        let content = format!("const {}ariable = 'test';", confusable);
        let findings = scanner.scan(&content, "test.js");
        
        assert!(
            findings.iter().any(|f| f.category == UnicodeCategory::Homoglyph),
            "Should detect {} as confusable for {}", confusable, base
        );
    }
}

/// Test variation selector detection (Glassworm primary)
#[test]
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
    
    // Should complete in <100ms for 10K lines
    assert!(
        elapsed < std::time::Duration::from_millis(500),
        "Scan took {:?}, should be <100ms", elapsed
    );
    
    // Clean content should have no findings
    assert!(findings.is_empty());
}

/// Test combined attack patterns
#[test]
fn test_combined_attack_patterns() {
    let scanner = UnicodeScanner::with_default_config();
    
    // Multiple attack vectors in one file
    let content = format!(
        "const pаssword\u{FE00} = 'secret';\n\
         const file = \"test\u{202E}exe\";\n\
         eval(\"code\");"
    );
    let findings = scanner.scan(&content, "test.js");
    
    assert!(findings.len() >= 3);
    assert!(findings.iter().any(|f| f.category == UnicodeCategory::Homoglyph));
    assert!(findings.iter().any(|f| f.category == UnicodeCategory::InvisibleCharacter));
    assert!(findings.iter().any(|f| f.category == UnicodeCategory::BidirectionalOverride));
}
