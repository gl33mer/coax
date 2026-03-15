//! Application state management for Coax TUI

use chrono::{DateTime, Local};
use coax_scanner::ScanResult;
use std::path::PathBuf;

/// Main application state
pub struct App {
    /// Whether the application is running
    pub running: bool,
    
    /// Current view
    pub view: View,
    
    /// Path being scanned
    pub scan_path: PathBuf,
    
    /// All scan results
    pub scan_results: Vec<Finding>,
    
    /// Filtered results (after applying filters/search)
    pub filtered_results: Vec<Finding>,
    
    /// Currently selected index in the list
    pub selected_index: usize,
    
    /// Filter by severity (None = all)
    pub filter_severity: Option<Severity>,
    
    /// Search query
    pub search_query: String,
    
    /// Whether search mode is active
    pub search_mode: bool,
    
    /// Sort field
    pub sort_by: SortField,
    
    /// Sort order
    pub sort_order: SortOrder,
    
    /// Last scan time
    pub last_scan_time: Option<DateTime<Local>>,
    
    /// Scroll offset for the list view
    pub scroll_offset: usize,
    
    /// Status message (temporary)
    pub status_message: Option<String>,
    
    /// Help panel visibility
    pub show_help: bool,
    
    /// Previous view (for back navigation)
    pub previous_view: Option<View>,
}

/// Available views in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    /// Main dashboard with statistics
    Dashboard,
    
    /// List of all findings
    FindingList,
    
    /// Detail view of a single finding
    FindingDetail,
    
    /// Settings panel
    Settings,
    
    /// Help panel
    Help,
}

/// Severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl Severity {
    /// Parse severity from string
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" => Severity::Medium,
            "low" => Severity::Low,
            _ => Severity::Info,
        }
    }
    
    /// Convert severity to string
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Critical => "critical",
            Severity::High => "high",
            Severity::Medium => "medium",
            Severity::Low => "low",
            Severity::Info => "info",
        }
    }
}

/// Fields to sort by
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortField {
    /// Sort by severity
    Severity,
    /// Sort by file path
    File,
    /// Sort by line number
    Line,
    /// Sort by pattern name
    Pattern,
}

/// Sort order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}

/// A finding with all its details
#[derive(Debug, Clone)]
pub struct Finding {
    /// File path where the finding was found
    pub file: String,
    
    /// Line number (1-indexed)
    pub line: u32,
    
    /// Column number (1-indexed, if available)
    pub column: Option<u32>,
    
    /// Pattern name that matched
    pub pattern: String,
    
    /// Severity level
    pub severity: Severity,
    
    /// Recommendation for remediation
    pub recommendation: String,
    
    /// The actual line content (if available)
    pub line_content: Option<String>,
    
    /// Code context (surrounding lines)
    pub code_context: Option<CodeContext>,
    
    /// Confidence score (0-100)
    pub confidence: u8,
    
    /// Whether this finding has been ignored
    pub ignored: bool,
    
    /// Whether this finding is marked as false positive
    pub false_positive: bool,
    
    /// Additional notes
    pub notes: Option<String>,
}

/// Code context around a finding
#[derive(Debug, Clone)]
pub struct CodeContext {
    /// Lines before the finding
    pub before: Vec<ContextLine>,
    /// The finding line
    pub finding: ContextLine,
    /// Lines after the finding
    pub after: Vec<ContextLine>,
}

/// A single line in the code context
#[derive(Debug, Clone)]
pub struct ContextLine {
    /// Line number
    pub number: u32,
    /// Line content
    pub content: String,
}

impl From<&ScanResult> for Finding {
    fn from(result: &ScanResult) -> Self {
        Finding {
            file: result.file.display().to_string(),
            line: result.line,
            column: None,
            pattern: result.pattern.clone(),
            severity: Severity::from_str(&result.severity),
            recommendation: result.recommendation.clone(),
            line_content: result.line_content.clone(),
            code_context: None,
            confidence: 100,
            ignored: false,
            false_positive: false,
            notes: None,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}

impl App {
    /// Create a new application instance
    pub fn new(scan_path: PathBuf) -> Self {
        App {
            running: true,
            view: View::Dashboard,
            scan_path,
            scan_results: Vec::new(),
            filtered_results: Vec::new(),
            selected_index: 0,
            filter_severity: None,
            search_query: String::new(),
            search_mode: false,
            sort_by: SortField::Severity,
            sort_order: SortOrder::Desc,
            last_scan_time: None,
            scroll_offset: 0,
            status_message: None,
            show_help: false,
            previous_view: None,
        }
    }
    
    /// Run a scan on the configured path
    pub fn scan(&mut self) {
        use coax_scanner::{Scanner, ScannerConfig};
        
        self.status_message = Some("Scanning...".to_string());
        
        let config = ScannerConfig::default()
            .with_line_content();
        
        let scanner = Scanner::with_config(config);
        let results = scanner.scan_directory(&self.scan_path);
        
        self.scan_results = results
            .iter()
            .map(Finding::from)
            .collect();
        
        self.last_scan_time = Some(Local::now());
        self.apply_filters();
        
        self.status_message = Some(format!(
            "Scan complete: {} findings",
            self.scan_results.len()
        ));
    }
    
    /// Apply current filters and sorting to results
    pub fn apply_filters(&mut self) {
        // Start with all results
        let mut filtered: Vec<Finding> = self.scan_results.clone();
        
        // Apply severity filter
        if let Some(severity) = &self.filter_severity {
            filtered.retain(|f| &f.severity == severity);
        }
        
        // Apply search filter
        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            filtered.retain(|f| {
                f.file.to_lowercase().contains(&query)
                    || f.pattern.to_lowercase().contains(&query)
                    || f.recommendation.to_lowercase().contains(&query)
                    || f.line_content
                        .as_ref()
                        .map(|c| c.to_lowercase().contains(&query))
                        .unwrap_or(false)
            });
        }
        
        // Apply sorting
        filtered.sort_by(|a, b| {
            let cmp = match self.sort_by {
                SortField::Severity => {
                    let sev_a = severity_order(&a.severity);
                    let sev_b = severity_order(&b.severity);
                    sev_a.cmp(&sev_b)
                }
                SortField::File => a.file.cmp(&b.file),
                SortField::Line => {
                    if a.file == b.file {
                        a.line.cmp(&b.line)
                    } else {
                        a.file.cmp(&b.file)
                    }
                }
                SortField::Pattern => a.pattern.cmp(&b.pattern),
            };
            
            match self.sort_order {
                SortOrder::Asc => cmp,
                SortOrder::Desc => cmp.reverse(),
            }
        });
        
        self.filtered_results = filtered;
        
        // Adjust selection if needed
        if self.selected_index >= self.filtered_results.len() {
            self.selected_index = self.filtered_results.len().saturating_sub(1);
        }
    }
    
    /// Get the currently selected finding
    pub fn selected_finding(&self) -> Option<&Finding> {
        self.filtered_results.get(self.selected_index)
    }
    
    /// Navigate up in the list
    pub fn navigate_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            // Adjust scroll offset if needed
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }
    
    /// Navigate down in the list
    pub fn navigate_down(&mut self) {
        if self.selected_index < self.filtered_results.len().saturating_sub(1) {
            self.selected_index += 1;
            // Adjust scroll offset if needed
            // Will be handled by UI based on viewport
        }
    }
    
    /// Switch to a different view
    pub fn switch_view(&mut self, view: View) {
        if self.view != view {
            self.previous_view = Some(self.view);
            self.view = view;
        }
    }
    
    /// Go back to the previous view
    pub fn go_back(&mut self) {
        if let Some(prev) = self.previous_view.take() {
            self.view = prev;
        }
    }
    
    /// Toggle the severity filter
    pub fn toggle_severity_filter(&mut self, severity: Option<Severity>) {
        self.filter_severity = severity;
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.apply_filters();
    }
    
    /// Set the search query
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
        self.selected_index = 0;
        self.scroll_offset = 0;
        self.apply_filters();
    }
    
    /// Clear the search
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.search_mode = false;
        self.apply_filters();
    }
    
    /// Set sort field
    pub fn set_sort_field(&mut self, field: SortField) {
        if self.sort_by == field {
            // Toggle order if same field
            self.sort_order = match self.sort_order {
                SortOrder::Asc => SortOrder::Desc,
                SortOrder::Desc => SortOrder::Asc,
            };
        } else {
            self.sort_by = field;
            self.sort_order = SortOrder::Desc;
        }
        self.apply_filters();
    }
    
    /// Mark the selected finding as ignored
    pub fn ignore_selected(&mut self) {
        if let Some(finding) = self.scan_results.get_mut(self.selected_index) {
            finding.ignored = true;
            self.status_message = Some("Finding ignored".to_string());
        }
    }
    
    /// Mark the selected finding as false positive
    pub fn mark_false_positive(&mut self) {
        if let Some(finding) = self.scan_results.get_mut(self.selected_index) {
            finding.false_positive = true;
            self.status_message = Some("Marked as false positive".to_string());
        }
    }
    
    /// Get severity counts for dashboard
    pub fn severity_counts(&self) -> SeverityCounts {
        let mut counts = SeverityCounts::default();
        for result in &self.scan_results {
            match result.severity {
                Severity::Critical => counts.critical += 1,
                Severity::High => counts.high += 1,
                Severity::Medium => counts.medium += 1,
                Severity::Low => counts.low += 1,
                Severity::Info => counts.info += 1,
            }
        }
        counts
    }
    
    /// Get recent findings (for dashboard)
    pub fn recent_findings(&self, limit: usize) -> Vec<&Finding> {
        self.filtered_results
            .iter()
            .take(limit)
            .collect()
    }
    
    /// Clear status message
    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}

/// Count of findings by severity
#[derive(Debug, Default, Clone)]
pub struct SeverityCounts {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
    pub info: u32,
}

/// Get numeric order for severity (for sorting)
fn severity_order(severity: &Severity) -> u8 {
    match severity {
        Severity::Critical => 5,
        Severity::High => 4,
        Severity::Medium => 3,
        Severity::Low => 2,
        Severity::Info => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_creation() {
        let app = App::new(PathBuf::from("."));
        assert!(app.running);
        assert_eq!(app.view, View::Dashboard);
        assert_eq!(app.scan_results.len(), 0);
    }
    
    #[test]
    fn test_navigation() {
        let mut app = App::new(PathBuf::from("."));
        
        // Add some test findings
        app.scan_results = vec![
            Finding {
                file: "test1.rs".to_string(),
                line: 1,
                column: None,
                pattern: "TEST".to_string(),
                severity: Severity::High,
                recommendation: "Test".to_string(),
                line_content: None,
                code_context: None,
                confidence: 100,
                ignored: false,
                false_positive: false,
                notes: None,
            },
            Finding {
                file: "test2.rs".to_string(),
                line: 2,
                column: None,
                pattern: "TEST2".to_string(),
                severity: Severity::Medium,
                recommendation: "Test2".to_string(),
                line_content: None,
                code_context: None,
                confidence: 100,
                ignored: false,
                false_positive: false,
                notes: None,
            },
        ];
        app.apply_filters();
        
        assert_eq!(app.selected_index, 0);
        app.navigate_down();
        assert_eq!(app.selected_index, 1);
        app.navigate_up();
        assert_eq!(app.selected_index, 0);
    }
    
    #[test]
    fn test_severity_filter() {
        let mut app = App::new(PathBuf::from("."));
        
        app.scan_results = vec![
            Finding {
                file: "test1.rs".to_string(),
                line: 1,
                column: None,
                pattern: "TEST".to_string(),
                severity: Severity::High,
                recommendation: "Test".to_string(),
                line_content: None,
                code_context: None,
                confidence: 100,
                ignored: false,
                false_positive: false,
                notes: None,
            },
            Finding {
                file: "test2.rs".to_string(),
                line: 2,
                column: None,
                pattern: "TEST2".to_string(),
                severity: Severity::Medium,
                recommendation: "Test2".to_string(),
                line_content: None,
                code_context: None,
                confidence: 100,
                ignored: false,
                false_positive: false,
                notes: None,
            },
        ];
        
        app.apply_filters();
        assert_eq!(app.filtered_results.len(), 2);
        
        app.toggle_severity_filter(Some(Severity::High));
        assert_eq!(app.filtered_results.len(), 1);
        assert_eq!(app.filtered_results[0].severity, Severity::High);
    }
    
    #[test]
    fn test_view_switching() {
        let mut app = App::new(PathBuf::from("."));
        
        assert_eq!(app.view, View::Dashboard);
        app.switch_view(View::FindingList);
        assert_eq!(app.view, View::FindingList);
        app.switch_view(View::FindingDetail);
        assert_eq!(app.view, View::FindingDetail);
        app.go_back();
        assert_eq!(app.view, View::FindingList);
    }
}
