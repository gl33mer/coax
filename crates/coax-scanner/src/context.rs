//! Context Detection and Exclusion Patterns
//!
//! This module provides:
//! - Context detection (comments, test files, documentation)
//! - Placeholder detection
//! - AWS example key detection
//! - Exclusion pattern management
//!
//! Based on QA feedback and research from GitGuardian, Aikido Security, and Betterleaks.

use regex::Regex;
use lazy_static::lazy_static;
use std::path::Path;
use crate::result::FindingContext;

lazy_static! {
    // Comment detection patterns for various languages
    static ref COMMENT_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"^\s*//").unwrap(),      // C++, Java, JavaScript, Go, Rust
        Regex::new(r"^\s*#").unwrap(),        // Python, Ruby, Shell, YAML
        Regex::new(r"^\s*/\*").unwrap(),      // C-style block comment start
        Regex::new(r"^\s*\*").unwrap(),       // C-style block comment continuation
        Regex::new(r"^\s*--").unwrap(),       // SQL, Lua, Haskell
        Regex::new(r"^\s*;").unwrap(),        // Lisp, Clojure
        Regex::new(r"^\s*<!--").unwrap(),     // HTML, XML comment
        Regex::new(r"^\s*\{#").unwrap(),      // Jinja, Twig comment
    ];

    // Placeholder patterns - these should NOT be flagged as real secrets
    // These patterns match the VALUE portion, not variable names
    static ref PLACEHOLDER_PATTERNS: Vec<Regex> = vec![
        // Match placeholder VALUES (quoted strings only)
        Regex::new(r#"[:=]\s*[\x27\x22]your[-_]?access[-_]?key[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]your[-_]?secret[-_]?key[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]your[-_]?api[-_]?key[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]your[-_]?password[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]your[-_]?token[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]xxx+[\x27\x22]"#).unwrap(),
        Regex::new(r#"(?i)[:=]\s*[\x27\x22]changeme[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]replace[-_]?me[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]insert[-_]?here[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]example[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]sample[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]test[-_]?key[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]fake[-_]?key[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]dummy[-_]?key[\x27\x22]"#).unwrap(),
        Regex::new(r#"[:=]\s*[\x27\x22]placeholder[\x27\x22]"#).unwrap(),
        // Also match unquoted but with word boundaries
        Regex::new(r#"[:=]\s*\byour\b"#).unwrap(),
    ];

    // AWS example keys (from AWS documentation)
    static ref AWS_EXAMPLE_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"AKIAIOSFODNN7EXAMPLE").unwrap(),
        Regex::new(r"wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY").unwrap(),
        Regex::new(r"(?i)examplekey").unwrap(),
    ];

    // Test file patterns
    static ref TEST_FILE_PATTERNS: Vec<Regex> = vec![
        Regex::new(r".*_test\.").unwrap(),      // Go, JS test files
        Regex::new(r".*\.test\.").unwrap(),     // JS test files
        Regex::new(r".*\.spec\.").unwrap(),     // JS spec files
        Regex::new(r"^test_.*").unwrap(),       // Python test files
        Regex::new(r"^spec_.*").unwrap(),       // Ruby spec files
        Regex::new(r".*_spec\.").unwrap(),      // Ruby spec files
    ];

    // Documentation file extensions
    static ref DOC_EXTENSIONS: Vec<&'static str> = vec![
        "md", "mdx", "rst", "txt", "adoc", "asciidoc",
    ];

    // Constant key name patterns (config keys, not secrets)
    // These match patterns like: SESSION_KEY = "ov_console_api_key"
    static ref CONSTANT_KEY_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"^[A-Z][A-Z0-9_]*_KEY\s*[:=]\s*[\x27\x22][a-z0-9_-]+[\x27\x22]").unwrap(),
        Regex::new(r"^[A-Z][A-Z0-9_]*_MODE\s*[:=]\s*[\x27\x22]").unwrap(),
        Regex::new(r"^[A-Z][A-Z0-9_]*_COLLAPSED\s*[:=]\s*[\x27\x22]").unwrap(),
    ];

    // Short password patterns (likely test/placeholder)
    static ref SHORT_PASSWORD_PATTERN: Regex = Regex::new(r"(?i)(password|passwd|pwd)\s*[:=]\s*[\x27\x22][^\x27\x22]{8,12}[\x27\x22]").unwrap();
}

/// Exclusion patterns for file and directory filtering
#[derive(Debug, Clone, Default)]
pub struct ExclusionPatterns {
    /// Directory names to exclude
    pub directories: Vec<String>,
    /// File name patterns to exclude
    pub file_names: Vec<String>,
    /// File extensions to exclude
    pub extensions: Vec<String>,
    /// Path patterns to exclude (glob-style)
    pub path_patterns: Vec<String>,
}

impl ExclusionPatterns {
    /// Create default exclusion patterns
    pub fn new() -> Self {
        Self {
            directories: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                "vendor".to_string(),
                ".venv".to_string(),
                "__pycache__".to_string(),
                ".tox".to_string(),
                "coverage".to_string(),
                ".coverage".to_string(),
                "htmlcov".to_string(),
                ".eggs".to_string(),
                "*.egg-info".to_string(),
                ".pytest_cache".to_string(),
                ".mypy_cache".to_string(),
                ".ruff_cache".to_string(),
            ],
            file_names: vec![
                "Cargo.lock".to_string(),
                "package-lock.json".to_string(),
                "yarn.lock".to_string(),
                "pnpm-lock.yaml".to_string(),
                "go.sum".to_string(),
                "Gemfile.lock".to_string(),
                "composer.lock".to_string(),
                "*.min.js".to_string(),
                "*.bundle.js".to_string(),
                "*.lock".to_string(),
            ],
            extensions: vec![
                "lock".to_string(),
                "sum".to_string(),
                "map".to_string(),  // Source maps
            ],
            path_patterns: vec![
                "**/test/**".to_string(),
                "**/tests/**".to_string(),
                "**/__tests__/**".to_string(),
                "**/spec/**".to_string(),
                "**/fixtures/**".to_string(),
                "**/mocks/**".to_string(),
                "**/examples/**".to_string(),
                "**/example/**".to_string(),
                "**/docs/**".to_string(),
                "**/documentation/**".to_string(),
            ],
        }
    }

    /// Check if a path should be excluded
    pub fn should_exclude(&self, path: &Path) -> bool {
        // Check if any path segment matches a directory pattern (e.g., .git/config -> .git)
        let path_str = path.to_string_lossy();
        for dir in &self.directories {
            if path_str.starts_with(&format!("{}/", dir)) || 
               path_str.contains(&format!("/{}/", dir)) ||
               path_str == dir.as_str() {
                return true;
            }
        }
        
        // Check file_name against file_names patterns
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            for pattern in &self.file_names {
                if pattern.starts_with("*.") {
                    // Extension pattern
                    if name.ends_with(&pattern[1..]) {
                        return true;
                    }
                } else if name == pattern {
                    return true;
                }
            }
        }

        // Check extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if self.extensions.iter().any(|e| e == ext) {
                return true;
            }
        }

        // Check path patterns (glob-style matching)
        let path_str = path.to_string_lossy();
        for pattern in &self.path_patterns {
            // Convert glob pattern to simple substring match
            // **/tests/** -> tests (matches /tests/ or tests/)
            let glob_pattern = pattern.replace("**/", "");
            if glob_pattern.ends_with("/**") {
                // Pattern like "tests/**" - match if path starts with or contains the directory
                let dir_name = glob_pattern.replace("/**", "");
                if path_str.starts_with(&format!("{}/", dir_name)) ||
                   path_str.contains(&format!("/{}/", dir_name)) {
                    return true;
                }
            } else if path_str.contains(&glob_pattern) {
                return true;
            }
        }

        false
    }

    /// Add a directory to exclude
    pub fn with_directory(mut self, dir: &str) -> Self {
        self.directories.push(dir.to_string());
        self
    }

    /// Add a file pattern to exclude
    pub fn with_file_pattern(mut self, pattern: &str) -> Self {
        self.file_names.push(pattern.to_string());
        self
    }

    /// Add an extension to exclude
    pub fn with_extension(mut self, ext: &str) -> Self {
        self.extensions.push(ext.to_string());
        self
    }
}

/// Context analyzer for determining if a finding is likely a false positive
#[derive(Debug, Clone)]
pub struct ContextAnalyzer {
    /// Whether to exclude test files entirely
    pub exclude_test_files: bool,
    /// Whether to exclude documentation files
    pub exclude_documentation: bool,
    /// Whether to exclude comments
    pub exclude_comments: bool,
    /// Whether to exclude placeholders
    pub exclude_placeholders: bool,
    /// Whether to exclude AWS example keys
    pub exclude_aws_examples: bool,
}

impl Default for ContextAnalyzer {
    fn default() -> Self {
        Self {
            exclude_test_files: true,
            exclude_documentation: true,
            exclude_comments: true,
            exclude_placeholders: true,
            exclude_aws_examples: true,
        }
    }
}

impl ContextAnalyzer {
    /// Create a new context analyzer with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyze a line and file path to determine context
    pub fn analyze(&self, line: &str, file_path: &Path) -> FindingContext {
        let mut context = FindingContext::default();

        // Check if it's a comment
        context.is_comment = COMMENT_PATTERNS.iter().any(|p| p.is_match(line));

        // Check if it's a test file
        context.is_test_file = self.is_test_file(file_path);

        // Check if it's documentation
        context.is_documentation = self.is_documentation(file_path);

        // Check if it's a placeholder
        context.is_placeholder = PLACEHOLDER_PATTERNS.iter().any(|p| p.is_match(line));

        // Check if it's an AWS example key
        context.is_aws_example = AWS_EXAMPLE_PATTERNS.iter().any(|p| p.is_match(line));

        // Check for constant key names (config keys, not secrets)
        let is_constant_key = CONSTANT_KEY_PATTERNS.iter().any(|p| p.is_match(line));

        // Determine adjusted severity and notes
        self.adjust_severity(&mut context, line, is_constant_key);

        context
    }

    /// Check if a file is a test file
    pub fn is_test_file(&self, path: &Path) -> bool {
        // Check file name patterns
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if TEST_FILE_PATTERNS.iter().any(|p| p.is_match(name)) {
                return true;
            }
        }

        // Check if in test directory (must be whole path segments, not substrings)
        if let Some(parent) = path.parent() {
            let path_str = parent.to_string_lossy();
            // Use word boundary checks to avoid matching /tmp/test*
            if path_str.contains("/test/") ||
               path_str.contains("/tests/") ||
               path_str.contains("/__tests__/") ||
               path_str.contains("/spec/") ||
               path_str.contains("/fixtures/") ||
               path_str.ends_with("/test") ||
               path_str.ends_with("/tests") ||
               path_str.ends_with("/spec") {
                return true;
            }
        }

        false
    }

    /// Check if a file is documentation
    pub fn is_documentation(&self, path: &Path) -> bool {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return DOC_EXTENSIONS.iter().any(|&e| e == ext);
        }
        false
    }

    /// Check if a line contains a placeholder
    pub fn is_placeholder(&self, line: &str) -> bool {
        PLACEHOLDER_PATTERNS.iter().any(|p| p.is_match(line))
    }

    /// Check if a line contains an AWS example key
    pub fn is_aws_example(&self, line: &str) -> bool {
        AWS_EXAMPLE_PATTERNS.iter().any(|p| p.is_match(line))
    }

    /// Check if a line is a comment
    pub fn is_comment(&self, line: &str) -> bool {
        COMMENT_PATTERNS.iter().any(|p| p.is_match(line))
    }

    /// Check if a line is a constant key name (not a secret)
    pub fn is_constant_key_name(&self, line: &str) -> bool {
        CONSTANT_KEY_PATTERNS.iter().any(|p| p.is_match(line))
    }

    /// Adjust severity based on context
    fn adjust_severity(&self, context: &mut FindingContext, line: &str, is_constant_key: bool) {
        // AWS example keys should be excluded entirely
        if context.is_aws_example {
            context.adjusted_severity = Some("excluded".to_string());
            context.note = Some("AWS example key from documentation".to_string());
            return;
        }

        // Placeholders should be excluded
        if context.is_placeholder {
            context.adjusted_severity = Some("excluded".to_string());
            context.note = Some("Placeholder or example value".to_string());
            return;
        }

        // Constant key names (like SESSION_KEY = "ov_console_api_key") are not secrets
        if is_constant_key {
            context.adjusted_severity = Some("excluded".to_string());
            context.note = Some("Configuration key name, not a secret".to_string());
            return;
        }

        // Comments in source code - lower severity or exclude
        if context.is_comment {
            if self.exclude_comments {
                context.adjusted_severity = Some("excluded".to_string());
                context.note = Some("Found in comment".to_string());
            } else {
                context.adjusted_severity = Some("low".to_string());
                context.note = Some("Found in comment - may be documentation".to_string());
            }
            return;
        }

        // Test files - lower severity
        if context.is_test_file {
            if self.exclude_test_files {
                context.adjusted_severity = Some("excluded".to_string());
                context.note = Some("Found in test file".to_string());
            } else {
                context.adjusted_severity = Some("low".to_string());
                context.note = Some("Found in test file - may be test data".to_string());
            }
            return;
        }

        // Documentation files - lower severity or exclude
        if context.is_documentation {
            if self.exclude_documentation {
                context.adjusted_severity = Some("excluded".to_string());
                context.note = Some("Found in documentation file".to_string());
            } else {
                context.adjusted_severity = Some("low".to_string());
                context.note = Some("Found in documentation - may be example".to_string());
            }
            return;
        }
    }

    /// Determine if a finding should be excluded based on context
    pub fn should_exclude(&self, context: &FindingContext) -> bool {
        context.adjusted_severity.as_deref() == Some("excluded")
    }

    /// Get the final severity after context adjustment
    pub fn get_final_severity(&self, original_severity: &str, context: &FindingContext) -> String {
        if let Some(adjusted) = &context.adjusted_severity {
            if adjusted != "excluded" {
                return adjusted.clone();
            }
        }
        original_severity.to_string()
    }
}

/// Extract the actual secret value from a matched line
pub fn extract_secret(line: &str, pattern_name: &str) -> Option<String> {
    // For key=value patterns, extract the value
    if let Some(eq_pos) = line.find('=') {
        let value_part = line[eq_pos + 1..].trim();
        // Remove quotes
        let value = value_part.trim_matches(|c| c == '"' || c == '\'');
        if !value.is_empty() && !is_placeholder_value(value) {
            return Some(mask_secret(value));
        }
    }

    // For colon-separated patterns
    if let Some(colon_pos) = line.find(':') {
        let value_part = line[colon_pos + 1..].trim();
        let value = value_part.trim_matches(|c| c == '"' || c == '\'');
        if !value.is_empty() && !is_placeholder_value(value) {
            return Some(mask_secret(value));
        }
    }

    // For direct pattern matches (like AWS keys), return the matched portion
    if pattern_name.contains("AWS") || pattern_name.contains("GITHUB") || pattern_name.contains("STRIPE") {
        // Find the actual secret in the line
        for word in line.split_whitespace() {
            let clean = word.trim_matches(|c| c == '"' || c == '\'' || c == ',' || c == ';');
            if clean.len() >= 16 && !is_placeholder_value(clean) {
                return Some(mask_secret(clean));
            }
        }
    }

    None
}

/// Check if a value is a placeholder
fn is_placeholder_value(value: &str) -> bool {
    let lower = value.to_lowercase();
    
    // AWS key format (starts with AKIA) should not be treated as placeholder
    // even if it contains "EXAMPLE" (like AWS documentation keys)
    if lower.starts_with("akia") {
        return false;
    }
    
    // Check for specific placeholder patterns
    lower.contains("your-") ||
    lower.contains("your_") ||
    lower == "xxx" ||
    lower == "changeme" ||
    lower == "example" ||
    lower.contains("placeholder") ||
    lower.contains("test-") ||
    lower.contains("test_")
}

/// Mask a secret for safe display (UTF-8 safe)
pub fn mask_secret(secret: &str) -> String {
    let chars: Vec<char> = secret.chars().collect();
    let len = chars.len();
    
    if len <= 8 {
        return secret.to_string();
    }

    // Show first 4 and last 4 characters (char-based, not byte-based)
    let visible_start = 4;
    let visible_end = 4;

    let masked_len = len - visible_start - visible_end;
    let masked = "*".repeat(masked_len);

    let start: String = chars[..visible_start].iter().collect();
    let end: String = chars[len - visible_end..].iter().collect();
    format!("{}{}{}", start, masked, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_detection() {
        let analyzer = ContextAnalyzer::new();

        assert!(analyzer.is_comment("// AWS_KEY=AKIAIOSFODNN7EXAMPLE"));
        assert!(analyzer.is_comment("# AWS_KEY=AKIAIOSFODNN7EXAMPLE"));
        assert!(analyzer.is_comment("/* AWS_KEY=AKIAIOSFODNN7EXAMPLE"));
        assert!(!analyzer.is_comment("AWS_KEY=AKIAIOSFODNN7EXAMPLE"));
    }

    #[test]
    fn test_placeholder_detection() {
        let analyzer = ContextAnalyzer::new();

        assert!(analyzer.is_placeholder(r#"access_key="your-access-key""#));
        assert!(analyzer.is_placeholder(r#"secret_key="your-secret-key""#));
        assert!(analyzer.is_placeholder(r#"api_key="xxx""#));
        assert!(analyzer.is_placeholder(r#"token="CHANGEME""#));
        assert!(!analyzer.is_placeholder(r#"api_key="sk_live_1234567890""#));
    }

    #[test]
    fn test_aws_example_detection() {
        let analyzer = ContextAnalyzer::new();

        assert!(analyzer.is_aws_example("AKIAIOSFODNN7EXAMPLE"));
        assert!(analyzer.is_aws_example("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"));
        assert!(!analyzer.is_aws_example("AKIAIOSFODNN7REALKEY1"));
    }

    #[test]
    fn test_constant_key_name_detection() {
        let analyzer = ContextAnalyzer::new();

        assert!(analyzer.is_constant_key_name(r#"SESSION_KEY = "ov_console_api_key""#));
        assert!(analyzer.is_constant_key_name(r#"THEME_MODE_KEY = "ov_console_theme_mode""#));
        assert!(analyzer.is_constant_key_name(r#"NAV_COLLAPSED_KEY = "ov_console_nav_collapsed""#));
        assert!(!analyzer.is_constant_key_name(r#"AWS_SECRET_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY""#));
    }

    #[test]
    fn test_test_file_detection() {
        let analyzer = ContextAnalyzer::new();

        assert!(analyzer.is_test_file(Path::new("test_example.py")));
        assert!(analyzer.is_test_file(Path::new("example_test.py")));
        assert!(analyzer.is_test_file(Path::new("example.test.js")));
        assert!(analyzer.is_test_file(Path::new("tests/test_file.py")));
        assert!(analyzer.is_test_file(Path::new("__tests__/test_file.js")));
        assert!(!analyzer.is_test_file(Path::new("src/main.py")));
    }

    #[test]
    fn test_documentation_detection() {
        let analyzer = ContextAnalyzer::new();

        assert!(analyzer.is_documentation(Path::new("README.md")));
        assert!(analyzer.is_documentation(Path::new("docs/guide.rst")));
        assert!(analyzer.is_documentation(Path::new("LICENSE.txt")));
        assert!(!analyzer.is_documentation(Path::new("src/main.py")));
    }

    #[test]
    fn test_secret_masking() {
        // "AKIAIOSFODNN7EXAMPLE123" has 23 chars: AKIA + 15 asterisks + E123
        assert_eq!(mask_secret("AKIAIOSFODNN7EXAMPLE123"), "AKIA***************E123");
        // "ghp_1234567890abcdefghij1234567890abcdefghij" has 44 chars: ghp_ + 36 asterisks + ghij
        assert_eq!(mask_secret("ghp_1234567890abcdefghij1234567890abcdefghij"), "ghp_************************************ghij");
        assert_eq!(mask_secret("short"), "short");
    }

    #[test]
    fn test_unicode_handling() {
        // Regression test for UTF-8 crash bug
        // Previously crashed with: byte index 40 is not a char boundary; it is inside '）' (bytes 38..41)
        
        // Test masking with multi-byte UTF-8 characters (Chinese, Japanese, Korean)
        let chinese_secret = "你好 AKIAIOSFODNN7EXAMPLE 世界";
        let masked = mask_secret(chinese_secret);
        assert!(masked.contains("***")); // Should have masking
        assert!(!masked.is_empty());
        
        // Test extraction with Unicode comments
        assert_eq!(
            extract_secret(r#"AWS_KEY="AKIAIOSFODNN7EXAMPLE123"  # 中文注释"#, "AWS_ACCESS_KEY"),
            Some("AKIA***************E123".to_string())
        );
        
        // Test with Japanese characters
        assert_eq!(
            extract_secret(r#"api_key=sk_live_1234567890abcdefghij  # 日本語コメント"#, "STRIPE_SECRET_KEY"),
            Some("sk_l******************************ghij".to_string())
        );
        
        // Test with Korean characters  
        assert_eq!(
            extract_secret(r#"token=ghp_1234567890abcdefghij1234567890abcdefghij  # 한국어 주석"#, "GITHUB_PAT"),
            Some("ghp_************************************ghij".to_string())
        );
        
        // Test pure Unicode string masking
        let pure_unicode = "你好世界_test_secret_123_你好世界";
        let masked = mask_secret(pure_unicode);
        assert!(masked.contains("***"));
        
        // Should not panic on any UTF-8 input
        let result = mask_secret("🔑🔑🔑 super_secret_key 🔑🔑🔑");
        assert!(!result.is_empty());
    }

    #[test]
    fn test_secret_extraction() {
        // "AKIAIOSFODNN7EXAMPLE123" has 23 chars: AKIA + 15 asterisks + E123
        assert_eq!(
            extract_secret(r#"AWS_KEY="AKIAIOSFODNN7EXAMPLE123""#, "AWS_ACCESS_KEY"),
            Some("AKIA***************E123".to_string())
        );
        // "sk_live_1234567890abcdefghij1234567890abcdefghij" has 48 chars: sk_l + 40 asterisks + ghij
        assert_eq!(
            extract_secret(r#"api_key=sk_live_1234567890abcdefghij1234567890abcdefghij"#, "STRIPE_SECRET_KEY"),
            Some("sk_l****************************************ghij".to_string())
        );
    }

    #[test]
    fn test_context_analysis() {
        let analyzer = ContextAnalyzer::new();

        // Comment should be excluded
        let context = analyzer.analyze("// AWS_KEY=AKIAIOSFODNN7EXAMPLE", Path::new("src/main.py"));
        assert!(context.is_comment);
        assert_eq!(context.adjusted_severity, Some("excluded".to_string()));

        // Real secret in source code should not be excluded
        let context = analyzer.analyze("AWS_KEY=AKIAIOSFODNN7REALKEY1", Path::new("src/main.py"));
        assert!(!context.is_comment);
        assert!(!context.is_placeholder);
        assert!(!context.is_aws_example);
        assert_ne!(context.adjusted_severity, Some("excluded".to_string()));
    }

    #[test]
    fn test_exclusion_patterns() {
        let exclusions = ExclusionPatterns::new();

        assert!(exclusions.should_exclude(Path::new(".git/config")));
        assert!(exclusions.should_exclude(Path::new("node_modules/package/index.js")));
        assert!(exclusions.should_exclude(Path::new("target/debug/main")));
        assert!(exclusions.should_exclude(Path::new("tests/test_file.py")));
        assert!(!exclusions.should_exclude(Path::new("src/main.py")));
    }
}
