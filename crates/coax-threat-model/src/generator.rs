//! Threat Model Generator
//!
//! This module provides threat model generation capabilities including:
//! - Entry point detection (HTTP routes, CLI commands, public functions)
//! - Trust boundary detection
//! - Data flow identification
//! - Threat generation from findings

use anyhow::Result;
use regex::Regex;
use std::path::Path;
use tracing::{debug, info};
use walkdir::WalkDir;

use crate::model::{
    Asset, AssetKind, DataFlow, EntryPoint, EntryPointKind, Sensitivity, Severity, Threat,
    ThreatModel, TrustBoundary, TrustBoundaryKind,
};
use crate::stride::{
    calculate_risk_score, categorize_finding_stride, determine_impact, determine_likelihood,
};
use coax_scanner::{ScanResult, Scanner};

/// Configuration for the threat model generator
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Scan hidden files and directories
    pub scan_hidden: bool,
    /// Maximum file size to analyze
    pub max_file_size: u64,
    /// Exclude patterns
    pub exclude_patterns: Vec<String>,
    /// File extensions to analyze
    pub extensions: Vec<String>,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            scan_hidden: false,
            max_file_size: 10 * 1024 * 1024, // 10MB
            exclude_patterns: vec![
                "node_modules".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                ".git".to_string(),
                "vendor".to_string(),
            ],
            extensions: vec![
                "rs".to_string(),
                "py".to_string(),
                "js".to_string(),
                "ts".to_string(),
                "go".to_string(),
                "java".to_string(),
                "c".to_string(),
                "cpp".to_string(),
                "h".to_string(),
                "rb".to_string(),
                "php".to_string(),
                "yml".to_string(),
                "yaml".to_string(),
                "json".to_string(),
                "toml".to_string(),
                "env".to_string(),
                "conf".to_string(),
                "config".to_string(),
            ],
        }
    }
}

/// Threat model generator
pub struct ThreatModelGenerator {
    config: GeneratorConfig,
    // Compiled regex patterns for detection
    entry_point_patterns: Vec<EntryPointPattern>,
    trust_boundary_patterns: Vec<TrustBoundaryPattern>,
    data_flow_patterns: Vec<DataFlowPattern>,
}

struct EntryPointPattern {
    name: &'static str,
    regex: Regex,
    kind: EntryPointKind,
    method_capture: Option<usize>,
    path_capture: usize,
}

struct TrustBoundaryPattern {
    name: &'static str,
    regex: Regex,
    kind: TrustBoundaryKind,
}

struct DataFlowPattern {
    name: &'static str,
    regex: Regex,
    data_type: &'static str,
    protocol: Option<&'static str>,
}

impl ThreatModelGenerator {
    /// Create a new generator with default configuration
    pub fn new() -> Self {
        Self::with_config(GeneratorConfig::default())
    }

    /// Create a new generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Self {
        let entry_point_patterns = Self::build_entry_point_patterns();
        let trust_boundary_patterns = Self::build_trust_boundary_patterns();
        let data_flow_patterns = Self::build_data_flow_patterns();

        Self {
            config,
            entry_point_patterns,
            trust_boundary_patterns,
            data_flow_patterns,
        }
    }

    /// Build entry point detection patterns
    fn build_entry_point_patterns() -> Vec<EntryPointPattern> {
        vec![
            // Express.js routes
            EntryPointPattern {
                name: "express_get",
                regex: Regex::new(r#"app\.get\s*\(\s*["']([^"']+)["']"#).unwrap(),
                kind: EntryPointKind::Http,
                method_capture: None,
                path_capture: 1,
            },
            EntryPointPattern {
                name: "express_post",
                regex: Regex::new(r#"app\.post\s*\(\s*["']([^"']+)["']"#).unwrap(),
                kind: EntryPointKind::Http,
                method_capture: None,
                path_capture: 1,
            },
            EntryPointPattern {
                name: "router_get",
                regex: Regex::new(r#"router\.(get|post|put|delete|patch)\s*\(\s*["']([^"']+)["']"#)
                    .unwrap(),
                kind: EntryPointKind::Http,
                method_capture: Some(1),
                path_capture: 2,
            },
            // Flask routes
            EntryPointPattern {
                name: "flask_route",
                regex: Regex::new(r#"@(?:app|blueprint)\.route\s*\(\s*["']([^"']+)["']"#).unwrap(),
                kind: EntryPointKind::Http,
                method_capture: None,
                path_capture: 1,
            },
            EntryPointPattern {
                name: "flask_method",
                regex: Regex::new(
                    r#"@(?:app|blueprint)\.(get|post|put|delete|patch)\s*\(\s*["']([^"']+)["']"#,
                )
                .unwrap(),
                kind: EntryPointKind::Http,
                method_capture: Some(1),
                path_capture: 2,
            },
            // FastAPI routes
            EntryPointPattern {
                name: "fastapi_route",
                regex: Regex::new(
                    r#"@(?:app|router)\.(get|post|put|delete|patch)\s*\(\s*["']([^"']+)["']"#,
                )
                .unwrap(),
                kind: EntryPointKind::Http,
                method_capture: Some(1),
                path_capture: 2,
            },
            // Axum routes
            EntryPointPattern {
                name: "axum_route",
                regex: Regex::new(r#"\.route\s*\(\s*["']([^"']+)["']"#).unwrap(),
                kind: EntryPointKind::Http,
                method_capture: None,
                path_capture: 1,
            },
            // Actix routes
            EntryPointPattern {
                name: "actix_get",
                regex: Regex::new(r#"#\[get\s*\(\s*["']([^"']+)["']\s*\)\]"#).unwrap(),
                kind: EntryPointKind::Http,
                method_capture: None,
                path_capture: 1,
            },
            EntryPointPattern {
                name: "actix_post",
                regex: Regex::new(r#"#\[post\s*\(\s*["']([^"']+)["']\s*\)\]"#).unwrap(),
                kind: EntryPointKind::Http,
                method_capture: None,
                path_capture: 1,
            },
            // CLI commands (clap)
            EntryPointPattern {
                name: "clap_command",
                regex: Regex::new(r#"#\[command\(name\s*=\s*["']([^"']+)["']\)\]"#).unwrap(),
                kind: EntryPointKind::Cli,
                method_capture: None,
                path_capture: 1,
            },
            // GraphQL endpoints
            EntryPointPattern {
                name: "graphql_query",
                regex: Regex::new(r"(?i)query\s+\w+\s*\{").unwrap(),
                kind: EntryPointKind::GraphQl,
                method_capture: None,
                path_capture: 0,
            },
            EntryPointPattern {
                name: "graphql_mutation",
                regex: Regex::new(r"(?i)mutation\s+\w+\s*\{").unwrap(),
                kind: EntryPointKind::GraphQl,
                method_capture: None,
                path_capture: 0,
            },
        ]
    }

    /// Build trust boundary detection patterns
    fn build_trust_boundary_patterns() -> Vec<TrustBoundaryPattern> {
        vec![
            // Authentication middleware
            TrustBoundaryPattern {
                name: "auth_middleware",
                regex: Regex::new(r"(?i)(auth|authentication|authorize|permission|middleware)")
                    .unwrap(),
                kind: TrustBoundaryKind::Auth,
            },
            // Network boundaries
            TrustBoundaryPattern {
                name: "vpc",
                regex: Regex::new(r"(?i)(vpc|subnet|firewall|security.?group)").unwrap(),
                kind: TrustBoundaryKind::Network,
            },
            // Container boundaries
            TrustBoundaryPattern {
                name: "container",
                regex: Regex::new(r"(?i)(docker|container|kubernetes|k8s|pod|deployment)").unwrap(),
                kind: TrustBoundaryKind::Process,
            },
            // Encryption boundaries
            TrustBoundaryPattern {
                name: "tls",
                regex: Regex::new(r"(?i)(tls|ssl|https|encrypted|cipher|aes|encrypt)").unwrap(),
                kind: TrustBoundaryKind::Encryption,
            },
            // Data classification
            TrustBoundaryPattern {
                name: "pii",
                regex: Regex::new(r"(?i)(pii|personal.?data|sensitive|confidential|gdpr)").unwrap(),
                kind: TrustBoundaryKind::DataClassification,
            },
        ]
    }

    /// Build data flow detection patterns
    fn build_data_flow_patterns() -> Vec<DataFlowPattern> {
        vec![
            // Database connections
            DataFlowPattern {
                name: "postgres_connect",
                regex: Regex::new(r"(?i)(postgres|pg|postgresql)\.?(connect|query|execute|new)")
                    .unwrap(),
                data_type: "Database Query",
                protocol: Some("TCP"),
            },
            DataFlowPattern {
                name: "mysql_connect",
                regex: Regex::new(r"(?i)(mysql|mysqli)\.?(connect|query|execute|new)").unwrap(),
                data_type: "Database Query",
                protocol: Some("TCP"),
            },
            DataFlowPattern {
                name: "mongodb_connect",
                regex: Regex::new(r"(?i)(mongo|mongodb)\.?(connect|find|insert|update)").unwrap(),
                data_type: "Database Query",
                protocol: Some("TCP"),
            },
            // API calls
            DataFlowPattern {
                name: "http_fetch",
                regex: Regex::new(
                    r"(?i)(fetch|axios|http|requests)\.?(get|post|put|delete|request)",
                )
                .unwrap(),
                data_type: "HTTP Request",
                protocol: Some("HTTP"),
            },
            // File I/O
            DataFlowPattern {
                name: "file_read",
                regex: Regex::new(r"(?i)(fs\.read|file\.read|open|read_file)").unwrap(),
                data_type: "File Content",
                protocol: None,
            },
            DataFlowPattern {
                name: "file_write",
                regex: Regex::new(r"(?i)(fs\.write|file\.write|write_file)").unwrap(),
                data_type: "File Content",
                protocol: None,
            },
            // Network
            DataFlowPattern {
                name: "socket_connect",
                regex: Regex::new(r"(?i)(socket|net|tcp)\.?(connect|dial|listen)").unwrap(),
                data_type: "Network Data",
                protocol: Some("TCP"),
            },
            // Redis
            DataFlowPattern {
                name: "redis_connect",
                regex: Regex::new(r"(?i)(redis)\.?(connect|get|set|del)").unwrap(),
                data_type: "Cache Data",
                protocol: Some("TCP"),
            },
        ]
    }

    /// Generate a threat model for the given path
    pub fn generate(&self, path: &Path) -> Result<ThreatModel> {
        info!("Generating threat model for: {:?}", path);

        // 1. Find entry points
        let entry_points = self.find_entry_points(path)?;
        debug!("Found {} entry points", entry_points.len());

        // 2. Detect trust boundaries
        let trust_boundaries = self.detect_trust_boundaries(path)?;
        debug!("Found {} trust boundaries", trust_boundaries.len());

        // 3. Identify data flows
        let data_flows = self.identify_data_flows(path)?;
        debug!("Found {} data flows", data_flows.len());

        // 4. Scan for secrets/vulnerabilities
        let findings = self.scan_findings(path)?;
        debug!("Found {} security findings", findings.len());

        // 5. Generate threats from findings
        let threats = self.generate_threats(&findings, &entry_points)?;
        debug!("Generated {} threats", threats.len());

        // 6. Identify assets
        let assets = self.identify_assets(&findings);
        debug!("Identified {} assets", assets.len());

        // 7. Build threat model
        let model = ThreatModel {
            repository: path.to_string_lossy().to_string(),
            generated_at: chrono::Utc::now(),
            entry_points,
            trust_boundaries,
            data_flows,
            threats,
            assets,
        };

        info!(
            "Threat model generated: {} threats, {} entry points, {} data flows",
            model.threats.len(),
            model.entry_points.len(),
            model.data_flows.len()
        );

        Ok(model)
    }

    /// Find entry points in the codebase
    pub fn find_entry_points(&self, path: &Path) -> Result<Vec<EntryPoint>> {
        let mut entry_points = Vec::new();

        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(path) {
                let eps =
                    self.detect_entry_points_in_content(&content, path.to_str().unwrap_or(""));
                entry_points.extend(eps);
            }
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| self.should_process_entry(e))
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let eps = self.detect_entry_points_in_content(
                            &content,
                            entry.path().to_str().unwrap_or(""),
                        );
                        entry_points.extend(eps);
                    }
                }
            }
        }

        Ok(entry_points)
    }

    /// Detect entry points in file content
    fn detect_entry_points_in_content(&self, content: &str, file_path: &str) -> Vec<EntryPoint> {
        let mut entry_points = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.entry_point_patterns {
                if let Some(captures) = pattern.regex.captures(line) {
                    let path = captures
                        .get(pattern.path_capture)
                        .map_or("", |m| m.as_str());
                    let method = pattern
                        .method_capture
                        .and_then(|i| captures.get(i))
                        .map(|m| m.as_str().to_uppercase());

                    // Determine method from pattern name if not captured
                    let method = method.or_else(|| {
                        if pattern.name.contains("get") {
                            Some("GET".to_string())
                        } else if pattern.name.contains("post") {
                            Some("POST".to_string())
                        } else if pattern.name.contains("put") {
                            Some("PUT".to_string())
                        } else if pattern.name.contains("delete") {
                            Some("DELETE".to_string())
                        } else {
                            None
                        }
                    });

                    // Check for authentication indicators
                    let has_auth = self.detect_authentication(line, content);

                    entry_points.push(EntryPoint {
                        name: format!("{} {}", method.as_deref().unwrap_or("Endpoint"), path),
                        kind: pattern.kind.clone(),
                        path: file_path.to_string(),
                        method,
                        authentication: has_auth,
                        inputs: Vec::new(), // Could be enhanced to extract parameters
                        line: Some(line_num as u32 + 1),
                    });
                }
            }
        }

        entry_points
    }

    /// Detect authentication indicators
    fn detect_authentication(&self, line: &str, content: &str) -> bool {
        let auth_patterns = [
            r"(?i)auth",
            r"(?i)middleware",
            r"(?i)guard",
            r"(?i)permission",
            r"(?i)authorize",
            r"(?i)requires.?auth",
            r"(?i)is.?authenticated",
        ];

        for pattern in &auth_patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(line) || re.is_match(content) {
                    return true;
                }
            }
        }
        false
    }

    /// Detect trust boundaries in the codebase
    pub fn detect_trust_boundaries(&self, path: &Path) -> Result<Vec<TrustBoundary>> {
        let mut boundaries_map: std::collections::HashMap<String, TrustBoundary> =
            std::collections::HashMap::new();

        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(path) {
                let boundaries =
                    self.detect_trust_boundaries_in_content(&content, path.to_str().unwrap_or(""));
                for b in boundaries {
                    boundaries_map.entry(b.name.clone()).or_insert(b);
                }
            }
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| self.should_process_entry(e))
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let boundaries = self.detect_trust_boundaries_in_content(
                            &content,
                            entry.path().to_str().unwrap_or(""),
                        );
                        for b in boundaries {
                            boundaries_map.entry(b.name.clone()).or_insert(b);
                        }
                    }
                }
            }
        }

        Ok(boundaries_map.into_values().collect())
    }

    /// Detect trust boundaries in file content
    fn detect_trust_boundaries_in_content(
        &self,
        content: &str,
        file_path: &str,
    ) -> Vec<TrustBoundary> {
        let mut boundaries = Vec::new();

        for pattern in &self.trust_boundary_patterns {
            if pattern.regex.is_match(content) {
                let component = file_path.to_string();
                boundaries.push(TrustBoundary {
                    name: format!("{} Boundary", pattern.kind),
                    kind: pattern.kind.clone(),
                    components: vec![component],
                    description: Some(format!("Detected via {}", pattern.name)),
                });
            }
        }

        boundaries
    }

    /// Identify data flows in the codebase
    pub fn identify_data_flows(&self, path: &Path) -> Result<Vec<DataFlow>> {
        let mut data_flows = Vec::new();

        if path.is_file() {
            if let Ok(content) = std::fs::read_to_string(path) {
                let flows =
                    self.detect_data_flows_in_content(&content, path.to_str().unwrap_or(""));
                data_flows.extend(flows);
            }
        } else if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_entry(|e| self.should_process_entry(e))
                .filter_map(|e| e.ok())
            {
                if entry.path().is_file() {
                    if let Ok(content) = std::fs::read_to_string(entry.path()) {
                        let flows = self.detect_data_flows_in_content(
                            &content,
                            entry.path().to_str().unwrap_or(""),
                        );
                        data_flows.extend(flows);
                    }
                }
            }
        }

        Ok(data_flows)
    }

    /// Detect data flows in file content
    fn detect_data_flows_in_content(&self, content: &str, _file_path: &str) -> Vec<DataFlow> {
        let mut data_flows = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.data_flow_patterns {
                if pattern.regex.is_match(line) {
                    data_flows.push(DataFlow {
                        from: "Application".to_string(),
                        to: format!("External ({})", pattern.name),
                        data_type: pattern.data_type.to_string(),
                        protocol: pattern.protocol.map(String::from),
                        encrypted: self.detect_encryption(content),
                        description: Some(format!("Line {}: {}", line_num + 1, line.trim())),
                    });
                }
            }
        }

        data_flows
    }

    /// Detect encryption indicators
    fn detect_encryption(&self, content: &str) -> bool {
        let encryption_patterns = [
            r"(?i)https://",
            r"(?i)tls",
            r"(?i)ssl",
            r"(?i)encrypt",
            r"(?i)aes",
            r"(?i)cipher",
        ];

        for pattern in &encryption_patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(content) {
                    return true;
                }
            }
        }
        false
    }

    /// Scan for security findings
    pub fn scan_findings(&self, path: &Path) -> Result<Vec<ScanResult>> {
        let scanner = Scanner::with_default_patterns();
        Ok(scanner.scan_with_summary(path).0)
    }

    /// Generate threats from findings
    pub fn generate_threats(
        &self,
        findings: &[ScanResult],
        entry_points: &[EntryPoint],
    ) -> Result<Vec<Threat>> {
        let mut threats = Vec::new();

        for finding in findings {
            let stride_categories = categorize_finding_stride(&finding.pattern);

            if stride_categories.is_empty() {
                continue;
            }

            // Find related entry points
            let related_eps: Vec<&EntryPoint> = entry_points
                .iter()
                .filter(|ep| self.is_entry_point_related(&finding, ep))
                .collect();

            for stride in stride_categories {
                let likelihood = determine_likelihood(finding, !related_eps.is_empty());
                let impact = determine_impact(&finding.pattern);
                let risk_score = calculate_risk_score(likelihood, impact);

                let exploited_ep = related_eps.first().map(|ep| ep.name.clone());

                threats.push(Threat {
                    id: format!("THR-{:03}", threats.len() + 1),
                    title: format!("{} via {}", stride, finding.pattern),
                    description: format!(
                        "Exposed {} at {}:{} could lead to {} attack. {}",
                        finding.pattern,
                        finding.file.display(),
                        finding.line,
                        stride,
                        crate::stride::stride_description(stride)
                    ),
                    stride,
                    severity: match finding.severity.to_lowercase().as_str() {
                        "critical" => Severity::Critical,
                        "high" => Severity::High,
                        "medium" => Severity::Medium,
                        _ => Severity::Low,
                    },
                    likelihood,
                    impact,
                    risk_score,
                    affected_component: format!("{}:{}", finding.file.display(), finding.line),
                    mitigation: finding.recommendation.clone(),
                    cwe_id: None,
                    related_finding: Some(finding.pattern.clone()),
                    exploited_entry_point: exploited_ep,
                });
            }
        }

        Ok(threats)
    }

    /// Check if entry point is related to finding
    fn is_entry_point_related(&self, finding: &ScanResult, entry_point: &EntryPoint) -> bool {
        // Check if file paths are related
        let finding_path = finding.file.to_string_lossy();
        entry_point.path.contains(finding_path.as_ref())
            || finding_path.as_ref().contains(&entry_point.path)
    }

    /// Identify assets from findings
    pub fn identify_assets(&self, findings: &[ScanResult]) -> Vec<Asset> {
        let mut assets = Vec::new();

        for finding in findings {
            let (kind, sensitivity) = self.classify_asset(&finding.pattern);

            assets.push(Asset {
                name: finding.pattern.clone(),
                kind,
                description: format!("Detected at {}:{}", finding.file.display(), finding.line),
                sensitivity,
                location: Some(format!("{}:{}", finding.file.display(), finding.line)),
            });
        }

        assets
    }

    /// Classify asset from pattern
    fn classify_asset(&self, pattern: &str) -> (AssetKind, Sensitivity) {
        match pattern.to_uppercase().as_str() {
            p if p.contains("PASSWORD") => (AssetKind::Secret, Sensitivity::Restricted),
            p if p.contains("PRIVATE_KEY") => (AssetKind::PrivateKey, Sensitivity::Restricted),
            p if p.contains("AWS") => (AssetKind::ApiKey, Sensitivity::Restricted),
            p if p.contains("DATABASE") => (AssetKind::Database, Sensitivity::Confidential),
            p if p.contains("TOKEN") || p.contains("KEY") => {
                (AssetKind::ApiKey, Sensitivity::Confidential)
            }
            p if p.contains("SECRET") => (AssetKind::Secret, Sensitivity::Confidential),
            _ => (AssetKind::Secret, Sensitivity::Internal),
        }
    }

    /// Check if directory entry should be processed
    fn should_process_entry(&self, entry: &walkdir::DirEntry) -> bool {
        let path = entry.path();

        // Skip hidden files/dirs if configured
        if !self.config.scan_hidden {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    return false;
                }
            }
        }

        // Check exclude patterns
        for pattern in &self.config.exclude_patterns {
            if path.to_string_lossy().contains(pattern.as_str()) {
                return false;
            }
        }

        // Check file extension for files
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if !self.config.extensions.iter().any(|e| e == ext) {
                    return false;
                }
            }
        }

        // Check file size
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() > self.config.max_file_size {
                debug!("Skipping large file: {:?} ({} bytes)", path, metadata.len());
                return false;
            }
        }

        true
    }
}

impl Default for ThreatModelGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::StrideCategory;
    use coax_scanner::{FindingContext, VerificationStatus};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_generator_creation() {
        let generator = ThreatModelGenerator::new();
        assert!(!generator.entry_point_patterns.is_empty());
        assert!(!generator.trust_boundary_patterns.is_empty());
        assert!(!generator.data_flow_patterns.is_empty());
    }

    #[test]
    fn test_entry_point_detection_express() {
        let generator = ThreatModelGenerator::new();
        let content = r#"
            app.get("/api/users", (req, res) => {});
            app.post("/api/login", (req, res) => {});
            router.get("/api/products", (req, res) => {});
        "#;

        let entry_points = generator.detect_entry_points_in_content(content, "test.js");
        assert_eq!(entry_points.len(), 3);
        assert!(entry_points.iter().any(|ep| ep.name.contains("/api/users")));
        assert!(entry_points.iter().any(|ep| ep.name.contains("/api/login")));
    }

    #[test]
    fn test_entry_point_detection_flask() {
        let generator = ThreatModelGenerator::new();
        let content = r#"
            @app.route("/api/users")
            def get_users():
                pass

            @app.post("/api/login")
            def login():
                pass
        "#;

        let entry_points = generator.detect_entry_points_in_content(content, "test.py");
        assert!(!entry_points.is_empty());
    }

    #[test]
    fn test_trust_boundary_detection() {
        let generator = ThreatModelGenerator::new();
        let content = r#"
            app.use(authMiddleware);
            // VPC configuration
            // Docker container setup
        "#;

        let boundaries = generator.detect_trust_boundaries_in_content(content, "test.js");
        assert!(!boundaries.is_empty());
    }

    #[test]
    fn test_data_flow_detection() {
        let generator = ThreatModelGenerator::new();
        let content = r#"
            const result = await db.query("SELECT * FROM users");
            const response = await fetch("https://api.example.com");
            fs.readFile("config.json");
        "#;

        let flows = generator.detect_data_flows_in_content(content, "test.js");
        assert!(!flows.is_empty());
    }

    #[test]
    fn test_threat_generation() {
        let generator = ThreatModelGenerator::new();

        // Create a mock finding
        let findings = vec![ScanResult {
            file: PathBuf::from("test.rs"),
            line: 10,
            column: Some(5),
            pattern: "AWS_ACCESS_KEY".to_string(),
            severity: "critical".to_string(),
            recommendation: "Rotate immediately".to_string(),
            detected_secret: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
            line_content: None,
            context: FindingContext::default(),
            verification: VerificationStatus::Unverified,
            description: None,
            cwe_id: None,
        }];

        let entry_points = vec![];
        let threats = generator
            .generate_threats(&findings, &entry_points)
            .unwrap();

        assert!(!threats.is_empty());
        assert!(
            threats[0].stride == StrideCategory::Spoofing
                || threats[0].stride == StrideCategory::InformationDisclosure
        );
    }

    #[test]
    fn test_full_generation() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.js");
        fs::write(
            &test_file,
            r#"app.get("/api/users", (req, res) => {});
const AWS_KEY = "AKIAIOSFODNN7EXAMPLE";
app.use(authMiddleware);
"#,
        )
        .unwrap();

        let generator = ThreatModelGenerator::new();
        let model = generator.generate(temp_dir.path()).unwrap();

        // Verify model was generated
        assert_eq!(model.repository, temp_dir.path().to_string_lossy());
        // Generator successfully created a threat model
    }
}
