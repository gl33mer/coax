//! Coax CLI
//!
//! High-performance security scanner for detecting secrets and vulnerabilities.

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use coax_scanner::{Scanner, ScannerConfig, ScanResult, OutputFormat, sarif_output, baseline::{BaselineFile, default_baseline_path, compare_with_baseline}};
use std::path::PathBuf;
use std::time::Instant;
use std::fs;
use std::process::Command;

/// Coax - High-Performance Security Scanner
#[derive(Parser, Debug)]
#[command(name = "coax")]
#[command(author = "Coax Team")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Coax secrets and vulnerabilities out of your codebases", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Quiet mode (minimal output)
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan for secrets and vulnerabilities
    Scan {
        /// Path to scan (file or directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: OutputFormatArg,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Number of threads (0 = auto)
        #[arg(short, long, default_value = "0")]
        threads: usize,

        /// Exclude patterns (comma-separated)
        #[arg(short, long)]
        exclude: Option<String>,

        /// Include line content in results
        #[arg(long)]
        with_content: bool,

        /// Scan hidden files and directories
        #[arg(long)]
        hidden: bool,

        /// Maximum file size to scan (e.g., "10MB", "1GB")
        #[arg(long, default_value = "10MB")]
        max_file_size: String,

        /// Path to baseline file (only report new findings)
        #[arg(long)]
        baseline: Option<PathBuf>,

        /// Scan only staged git files
        #[arg(long)]
        staged: bool,
    },

    /// Baseline management
    Baseline {
        #[command(subcommand)]
        action: BaselineAction,
    },

    /// Pre-commit hook management
    PreCommit {
        #[command(subcommand)]
        action: PreCommitAction,
    },

    /// Show version information
    Version,
}

#[derive(Subcommand, Debug)]
enum BaselineAction {
    /// Generate a new baseline file
    Generate {
        /// Path to scan
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output path for baseline file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Update existing baseline with new findings
    Update {
        /// Path to scan
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Path to baseline file
        #[arg(short, long)]
        baseline: Option<PathBuf>,
    },
}

#[derive(Subcommand, Debug)]
enum PreCommitAction {
    /// Install pre-commit hook
    Install,

    /// Uninstall pre-commit hook
    Uninstall,

    /// Run pre-commit scan manually
    Run,
}

#[derive(ValueEnum, Debug, Clone)]
enum OutputFormatArg {
    Text,
    Json,
    Yaml,
    Sarif,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Yaml => OutputFormat::Yaml,
            OutputFormatArg::Sarif => OutputFormat::Sarif,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    match cli.command {
        Commands::Scan {
            path,
            format,
            output,
            threads,
            exclude,
            with_content,
            hidden,
            max_file_size,
            baseline,
            staged,
        } => {
            if staged {
                run_staged_scan(
                    format.into(),
                    output,
                    threads,
                    exclude,
                    with_content,
                    hidden,
                    max_file_size,
                    baseline,
                )
            } else {
                run_scan(
                    path,
                    format.into(),
                    output,
                    threads,
                    exclude,
                    with_content,
                    hidden,
                    max_file_size,
                    baseline,
                )
            }
        }
        Commands::Baseline { action } => match action {
            BaselineAction::Generate { path, output } => run_baseline_generate(path, output),
            BaselineAction::Update { path, baseline } => run_baseline_update(path, baseline),
        },
        Commands::PreCommit { action } => match action {
            PreCommitAction::Install => run_precommit_install(),
            PreCommitAction::Uninstall => run_precommit_uninstall(),
            PreCommitAction::Run => run_precommit_run(),
        },
        Commands::Version => {
            println!("coax {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

/// Parse file size string (e.g., "10MB", "1GB") to bytes
fn parse_file_size(size: &str) -> Result<u64> {
    let size = size.trim().to_uppercase();
    let (num, suffix) = size.split_at(size.len() - 2);
    let num: u64 = num.trim().parse()?;

    let multiplier = match suffix.trim() {
        "KB" => 1024,
        "MB" => 1024 * 1024,
        "GB" => 1024 * 1024 * 1024,
        "B" => 1,
        _ => anyhow::bail!("Invalid size suffix: {}. Use B, KB, MB, or GB", suffix),
    };

    Ok(num * multiplier)
}

/// Get staged files from git
fn get_staged_files() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to get staged files");
    }

    let files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(files)
}

/// Run scan on staged files only
#[allow(clippy::too_many_arguments)]
fn run_staged_scan(
    format: OutputFormat,
    output: Option<PathBuf>,
    threads: usize,
    exclude: Option<String>,
    with_content: bool,
    hidden: bool,
    max_file_size: String,
    baseline: Option<PathBuf>,
) -> Result<()> {
    let staged_files = get_staged_files()?;

    if staged_files.is_empty() {
        eprintln!("{} No staged files to scan", "ℹ️".blue());
        return Ok(());
    }

    eprintln!(
        "{} Scanning {} staged files...",
        "🔍".blue(),
        staged_files.len().to_string().yellow()
    );

    // Create scanner and scan each file
    let mut config = ScannerConfig::default()
        .with_threads(threads)
        .with_max_file_size(parse_file_size(&max_file_size)?);

    if with_content {
        config = config.with_line_content();
    }

    if hidden {
        config.scan_hidden = true;
    }

    if let Some(exclude_str) = exclude {
        for pattern in exclude_str.split(',') {
            config.exclude_patterns.push(pattern.trim().to_string());
        }
    }

    let scanner = Scanner::with_config(config);
    let mut all_results = Vec::new();

    for file in &staged_files {
        let path = PathBuf::from(file);
        if path.exists() && path.is_file() {
            let results = scanner.scan_file(&path);
            all_results.extend(results);
        }
    }

    // Apply baseline filter if provided
    let results = if let Some(baseline_path) = baseline {
        if baseline_path.exists() {
            let baseline = BaselineFile::load(&baseline_path)?;
            baseline.filter_new_findings(&all_results)
        } else {
            all_results
        }
    } else {
        all_results
    };

    // Create summary
    let summary = coax_scanner::ScanSummary::from_results(&results);

    // Format and output
    let output_str = format_results(&results, &summary, format, std::time::Duration::from_millis(0))?;

    if let Some(output_path) = output {
        fs::write(&output_path, &output_str)?;
        eprintln!("{} Results written to {}", "✓".green(), output_path.display().to_string().cyan());
    } else {
        println!("{}", output_str);
    }

    // Exit with error if findings
    if !results.is_empty() {
        std::process::exit(1);
    }

    Ok(())
}

/// Run the security scan
#[allow(clippy::too_many_arguments)]
fn run_scan(
    path: PathBuf,
    format: OutputFormat,
    output: Option<PathBuf>,
    threads: usize,
    exclude: Option<String>,
    with_content: bool,
    hidden: bool,
    max_file_size: String,
    baseline: Option<PathBuf>,
) -> Result<()> {
    // Validate path
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Parse max file size
    let max_size_bytes = parse_file_size(&max_file_size)?;

    // Build configuration
    let mut config = ScannerConfig::default()
        .with_threads(threads)
        .with_max_file_size(max_size_bytes);

    if with_content {
        config = config.with_line_content();
    }

    if hidden {
        config.scan_hidden = true;
    }

    // Add exclude patterns
    if let Some(exclude_str) = exclude {
        for pattern in exclude_str.split(',') {
            config.exclude_patterns.push(pattern.trim().to_string());
        }
    }

    // Create scanner
    let scanner = Scanner::with_config(config);

    if !cli_quiet() {
        eprintln!(
            "{} {} - Scanning {}",
            "🔍".bold().blue(),
            "Coax".bold(),
            path.display().to_string().cyan()
        );
        eprintln!(
            "   {} patterns loaded",
            scanner.pattern_count().to_string().yellow()
        );
        eprintln!();
    }

    // Run scan
    let start = Instant::now();
    let (mut results, summary) = scanner.scan_with_summary(&path);
    let duration = start.elapsed();

    // Apply baseline filter if provided
    if let Some(baseline_path) = baseline {
        if baseline_path.exists() {
            let baseline = BaselineFile::load(&baseline_path)?;
            let new_findings = baseline.filter_new_findings(&results);
            
            if !cli_quiet() {
                eprintln!(
                    "{} Filtered {} findings ({} new, {} existing)",
                    "📊".blue(),
                    results.len(),
                    new_findings.len(),
                    results.len() - new_findings.len()
                );
            }
            
            results = new_findings;
        }
    }

    // Format output
    let output_str = format_results(&results, &summary, format, duration)?;

    // Write output
    if let Some(output_path) = output {
        fs::write(&output_path, &output_str)?;
        if !cli_quiet() {
            eprintln!(
                "{} Results written to {}",
                "✓".bold().green(),
                output_path.display().to_string().cyan()
            );
        }
    } else {
        println!("{}", output_str);
    }

    // Print summary to stderr if not quiet
    if !cli_quiet() && format != OutputFormat::Json && format != OutputFormat::Yaml && format != OutputFormat::Sarif {
        eprintln!();
        eprintln!("{}", "─".repeat(60).dimmed());
        eprintln!(
            "{} {} files in {:?}",
            "📊".bold().blue(),
            summary.files_scanned.to_string().yellow(),
            duration
        );
        eprintln!(
            "{} {} findings ({} critical, {} high, {} medium, {} low)",
            "🔍".bold().blue(),
            summary.total_findings.to_string().red().bold(),
            summary.by_severity.critical.to_string().red(),
            summary.by_severity.high.to_string().yellow(),
            summary.by_severity.medium.to_string().blue(),
            summary.by_severity.low.to_string().white()
        );

        if !summary.top_patterns.is_empty() {
            eprintln!();
            eprintln!("{}", "Top Patterns:".bold());
            for (i, pattern) in summary.top_patterns.iter().take(5).enumerate() {
                eprintln!(
                    "   {}. {}: {} {}",
                    i + 1,
                    pattern.pattern.yellow(),
                    pattern.count.to_string().cyan(),
                    "finding(s)".dimmed()
                );
            }
        }

        eprintln!("{}", "─".repeat(60).dimmed());
    }

    // Exit with error code if critical/high findings
    if summary.by_severity.critical > 0 || summary.by_severity.high > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Format results based on output format
fn format_results(
    results: &[ScanResult],
    summary: &coax_scanner::ScanSummary,
    format: OutputFormat,
    duration: std::time::Duration,
) -> Result<String> {
    match format {
        OutputFormat::Text => Ok(format_text(results, summary, duration)),
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "scan_duration_ms": duration.as_millis() as u64,
            "summary": summary,
            "findings": results
        }))?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&serde_yaml::Value::Mapping(
            serde_yaml::Mapping::from_iter([
                (
                    serde_yaml::Value::String("version".to_string()),
                    serde_yaml::Value::String(env!("CARGO_PKG_VERSION").to_string()),
                ),
                (
                    serde_yaml::Value::String("scan_duration_ms".to_string()),
                    serde_yaml::Value::Number((duration.as_millis() as u64).into()),
                ),
                (
                    serde_yaml::Value::String("summary".to_string()),
                    serde_yaml::to_value(summary)?,
                ),
                (
                    serde_yaml::Value::String("findings".to_string()),
                    serde_yaml::to_value(results)?,
                ),
            ]),
        ))?),
        OutputFormat::Sarif => Ok(sarif_output::generate_sarif_json(results, env!("CARGO_PKG_VERSION"))),
    }
}

/// Format results as text
fn format_text(
    results: &[ScanResult],
    _summary: &coax_scanner::ScanSummary,
    _duration: std::time::Duration,
) -> String {
    if results.is_empty() {
        return format!(
            "{}\n{}\n",
            "✓ No secrets or vulnerabilities detected".green().bold(),
            "Your code looks clean!".dimmed()
        );
    }

    let mut output = String::new();

    // Sort by severity (critical first)
    let mut sorted_results = results.to_vec();
    sorted_results.sort_by(|a, b| b.severity_score().cmp(&a.severity_score()));

    for result in &sorted_results {
        let severity_color = match result.severity.to_lowercase().as_str() {
            "critical" => |s: &str| s.red().bold(),
            "high" => |s: &str| s.yellow().bold(),
            "medium" => |s: &str| s.blue().bold(),
            "low" => |s: &str| s.white().bold(),
            _ => |s: &str| s.normal(),
        };

        let icon = match result.severity.to_lowercase().as_str() {
            "critical" => "🚨",
            "high" => "⚠️",
            "medium" => "⚡",
            "low" => "ℹ️",
            _ => "•",
        };

        // Show file:line:column
        let column_str = result.column.map(|c| format!(":{}", c)).unwrap_or_default();
        output.push_str(&format!(
            "{} {}:{}{} - {} [{}]\n",
            icon,
            result.file.display().to_string().cyan(),
            result.line.to_string().yellow(),
            column_str,
            result.pattern.yellow(),
            severity_color(&result.severity.to_uppercase()),
        ));

        // Show detected secret if available
        if let Some(secret) = &result.detected_secret {
            output.push_str(&format!(
                "   {} {}\n",
                "String:".bold(),
                secret.red()
            ));
        } else if let Some(content) = &result.line_content {
            output.push_str(&format!(
                "   {}\n",
                content.dimmed()
            ));
        }

        // Show recommendation
        output.push_str(&format!(
            "   {} {}\n",
            "Recommendation:".bold(),
            result.recommendation.dimmed()
        ));

        // Show context notes if available
        if let Some(note) = &result.context.note {
            output.push_str(&format!(
                "   {} {}\n",
                "Note:".bold(),
                note.italic().dimmed()
            ));
        }

        output.push('\n');
    }

    output
}

/// Generate baseline file
fn run_baseline_generate(path: PathBuf, output: Option<PathBuf>) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    eprintln!(
        "{} Generating baseline for {}",
        "🔍".blue(),
        path.display().to_string().cyan()
    );

    let scanner = Scanner::with_default_patterns();
    let (results, _) = scanner.scan_with_summary(&path);

    let mut baseline = BaselineFile::from_results(&results);

    let baseline_path = output.unwrap_or_else(|| default_baseline_path());
    baseline.save(&baseline_path)?;

    eprintln!(
        "{} Baseline generated: {} ({} findings)",
        "✓".green(),
        baseline_path.display().to_string().cyan(),
        baseline.findings.len().to_string().yellow()
    );

    Ok(())
}

/// Update baseline with new findings
fn run_baseline_update(path: PathBuf, baseline_path: Option<PathBuf>) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    let baseline_path = baseline_path.unwrap_or_else(|| default_baseline_path());

    if !baseline_path.exists() {
        anyhow::bail!("Baseline file not found: {}", baseline_path.display());
    }

    eprintln!(
        "{} Updating baseline {}",
        "🔍".blue(),
        baseline_path.display().to_string().cyan()
    );

    let mut baseline = BaselineFile::load(&baseline_path)?;
    let scanner = Scanner::with_default_patterns();
    let (results, _) = scanner.scan_with_summary(&path);

    let new_findings = baseline.update(&results);

    if new_findings.is_empty() {
        eprintln!("{} No new findings to add", "✓".green());
    } else {
        eprintln!(
            "{} Added {} new findings to baseline",
            "✓".green(),
            new_findings.len().to_string().yellow()
        );
    }

    baseline.save(&baseline_path)?;

    Ok(())
}

/// Install pre-commit hook
fn run_precommit_install() -> Result<()> {
    let git_hooks_dir = PathBuf::from(".git/hooks");
    
    if !git_hooks_dir.exists() {
        anyhow::bail!("Not a git repository. Run 'git init' first.");
    }

    // Find the pre-commit script in the coax installation
    let script_path = find_precommit_script()?;
    
    let hook_path = git_hooks_dir.join("pre-commit");
    
    // Copy the script
    fs::copy(&script_path, &hook_path)?;
    
    // Make it executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_path, perms)?;
    }

    eprintln!(
        "{} Pre-commit hook installed to {}",
        "✓".green(),
        hook_path.display().to_string().cyan()
    );

    Ok(())
}

/// Find the pre-commit script
fn find_precommit_script() -> Result<PathBuf> {
    // Try common locations
    let candidates = [
        PathBuf::from("scripts/pre-commit"),
        PathBuf::from("../scripts/pre-commit"),
        PathBuf::from("../../scripts/pre-commit"),
    ];

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }
    }

    anyhow::bail!(
        "Pre-commit script not found. Please ensure coax is properly installed."
    );
}

/// Uninstall pre-commit hook
fn run_precommit_uninstall() -> Result<()> {
    let hook_path = PathBuf::from(".git/hooks/pre-commit");

    if !hook_path.exists() {
        eprintln!("{} Pre-commit hook not found", "ℹ️".blue());
        return Ok(());
    }

    fs::remove_file(&hook_path)?;

    eprintln!(
        "{} Pre-commit hook removed from {}",
        "✓".green(),
        hook_path.display().to_string().cyan()
    );

    Ok(())
}

/// Run pre-commit scan manually
fn run_precommit_run() -> Result<()> {
    // This is essentially the same as staged scan
    run_staged_scan(
        OutputFormat::Text,
        None,
        0,
        None,
        false,
        false,
        "10MB".to_string(),
        Some(default_baseline_path()),
    )
}

/// Check if CLI should be quiet
fn cli_quiet() -> bool {
    // This would need to be passed from main, but for now we'll check env var
    std::env::var("COAX_QUIET").is_ok()
}
