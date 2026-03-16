// Test program to verify GitHistoryProvider functionality
// Run with: cargo run --bin test_git_history

use coax_scanner::source_provider::{GitHistoryProvider, SourceProvider, ContentLoader};
use std::path::PathBuf;

fn main() {
    println!("=== GitHistoryProvider Test ===\n");

    // Create provider for test repo
    let repo_path = PathBuf::from("/tmp/test-git-repo");
    let provider = GitHistoryProvider::new(&repo_path)
        .expect("Failed to open repository")
        .with_commit_limit(10)
        .with_scan_diffs(false); // Scan full files at each commit for thorough history scan

    println!("Repository: {:?}", repo_path);
    println!("Is shallow: {}", provider.is_shallow());
    println!("Progress unit: {}", provider.progress_unit_name());
    println!();

    // Enumerate targets
    println!("Enumerating git history...");
    let targets: Vec<_> = provider.enumerate().collect();
    println!("Found {} targets in git history\n", targets.len());

    // Show first few targets
    for (i, target) in targets.iter().take(5).enumerate() {
        println!("Target {}: {}", i + 1, target.display_path());
        println!("  Origin: {:?}", target.origin);
        println!("  Size hint: {:?}", target.size_hint);
        println!("  Content type: {:?}", target.content_type);
    }

    if targets.len() > 5 {
        println!("  ... and {} more", targets.len() - 5);
    }
    println!();

    // Load and scan content for secrets
    println!("Loading and scanning content for secrets...");
    let scanner = coax_scanner::Scanner::with_config(
        coax_scanner::ScannerConfig::default()
            .with_token_efficiency(false)
            .with_word_filter(false)
            .with_context_detection(false)
    );
    let mut total_findings = 0;

    for target in &targets {
        match provider.load(target) {
            Ok(content) => {
                if let Ok(text) = content.into_string() {
                    let findings = scanner.scan_content(&text, &target.display_path());
                    if !findings.is_empty() {
                        total_findings += findings.len();
                        println!("\nFound {} secret(s) in {}:", findings.len(), target.display_path());
                        for finding in &findings {
                            println!("  - Line {}: {} (Severity: {})", finding.line, finding.pattern, finding.severity);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to load {:?}: {}", target.display_path(), e);
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total targets scanned: {}", targets.len());
    println!("Total secrets found: {}", total_findings);

    if total_findings > 0 {
        println!("\n✅ GitHistoryProvider successfully found secrets in git history!");
    } else {
        println!("\n⚠️  No secrets found (this may be expected if filters are enabled)");
    }
}
