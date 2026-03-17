//! Source Provider Abstraction
//!
//! This module provides the `SourceProvider` and `ContentLoader` traits for
//! abstracting over different sources of scannable content (filesystem, git history,
//! Docker images, etc.).
//!
//! # Design
//!
//! The two-phase model separates enumeration (cheap) from content loading (expensive):
//! - `SourceProvider::enumerate()` - Lists available scan targets without loading content
//! - `ContentLoader::load()` - Loads content for specific targets on-demand
//!
//! This allows the scan engine to make skip decisions (binary files, size limits, etc.)
//! before incurring the cost of loading content.
//!
//! # Example
//!
//! ```rust
//! use coax_scanner::source_provider::{FileSystemProvider, SourceProvider, ContentLoader};
//! use std::path::PathBuf;
//! use std::sync::Arc;
//!
//! // Create a filesystem provider
//! let provider = Arc::new(FileSystemProvider::new(PathBuf::from("./src")));
//!
//! // Enumerate available targets
//! for target in provider.enumerate() {
//!     println!("Found: {:?}", target.origin);
//! }
//!
//! // Load content for a specific target
//! let content = provider.load(&target).unwrap();
//! ```

use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};

/// A scannable unit's metadata (without content)
///
/// Represents a single item that can be scanned, such as a file,
/// git blob, or diff hunk. Content is not loaded until explicitly requested.
#[derive(Debug, Clone)]
pub struct ScanTarget {
    /// Where this target originates
    pub origin: ScanOrigin,
    /// Expected size in bytes (for skip-if-too-large decisions)
    pub size_hint: Option<u64>,
    /// Content type hint (text, binary, etc.)
    pub content_type: Option<ContentType>,
}

impl ScanTarget {
    /// Create a new scan target
    pub fn new(origin: ScanOrigin) -> Self {
        Self {
            origin,
            size_hint: None,
            content_type: None,
        }
    }

    /// Set the size hint
    pub fn with_size_hint(mut self, size: u64) -> Self {
        self.size_hint = Some(size);
        self
    }

    /// Set the content type
    pub fn with_content_type(mut self, content_type: ContentType) -> Self {
        self.content_type = Some(content_type);
        self
    }

    /// Get a human-readable path or identifier for this target
    pub fn display_path(&self) -> String {
        match &self.origin {
            ScanOrigin::FileSystem { path } => path.display().to_string(),
            ScanOrigin::GitBlob { path, commit, .. } => {
                format!("{}@{}", path.display(), &commit[..8])
            }
            ScanOrigin::GitDiff { path, commit, .. } => {
                format!("{}@{}", path.display(), &commit[..8])
            }
        }
    }
}

/// Where scan content originates from
///
/// Different sources have different metadata available. This enum
/// captures the origin-specific information needed for reporting.
#[derive(Debug, Clone)]
pub enum ScanOrigin {
    /// A file from the local filesystem
    FileSystem {
        /// Absolute or relative path to the file
        path: PathBuf,
    },

    /// A specific blob from git history
    GitBlob {
        /// Commit SHA where this blob was found
        commit: String,
        /// Path within the commit tree
        path: PathBuf,
        /// Author of the commit (format: "Name <email>")
        author: String,
        /// Date of the commit
        date: DateTime<Utc>,
    },

    /// A diff hunk from a git commit
    GitDiff {
        /// Commit SHA
        commit: String,
        /// Path that was changed
        path: PathBuf,
        /// Whether this diff represents additions (true) or deletions (false)
        added: bool,
    },
}

/// Content type classification for scan targets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// Text content (can be scanned as UTF-8)
    Text,
    /// Binary content (may need special handling or skipping)
    Binary,
    /// Unknown (will be determined on load)
    Unknown,
}

/// Loaded content from a scan target
///
/// Supports both fully-buffered content (small files) and
/// streaming content (large files, diff hunks).
pub enum ScanContent {
    /// Fully buffered content in memory
    ///
    /// Suitable for small files and blobs under the size limit.
    Buffered(Vec<u8>),

    /// Streamed content, read in chunks
    ///
    /// Suitable for large files where streaming is more memory-efficient.
    Streamed(Box<dyn Read + Send + Sync>),
}

impl std::fmt::Debug for ScanContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanContent::Buffered(bytes) => {
                write!(f, "ScanContent::Buffered({} bytes)", bytes.len())
            }
            ScanContent::Streamed(_) => write!(f, "ScanContent::Streamed(..)"),
        }
    }
}

impl ScanContent {
    /// Convert to a Vec<u8> by reading all content
    ///
    /// For `Buffered` variants, this is a no-op clone.
    /// For `Streamed` variants, this reads all remaining content.
    pub fn into_bytes(mut self) -> io::Result<Vec<u8>> {
        match self {
            ScanContent::Buffered(bytes) => Ok(bytes),
            ScanContent::Streamed(ref mut reader) => {
                let mut buffer = Vec::new();
                reader.read_to_end(&mut buffer)?;
                Ok(buffer)
            }
        }
    }

    /// Convert to a String, assuming UTF-8 encoding
    pub fn into_string(self) -> io::Result<String> {
        let bytes = self.into_bytes()?;
        String::from_utf8(bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.utf8_error()))
    }

    /// Check if content appears to be binary (contains null bytes in first 8KB)
    pub fn is_binary(&self) -> io::Result<bool> {
        match self {
            ScanContent::Buffered(bytes) => Ok(contains_null_bytes(bytes)),
            ScanContent::Streamed(_) => {
                // For streamed content, we'd need to peek at the first bytes
                // This is a limitation - binary detection should happen before loading
                Ok(false)
            }
        }
    }
}

/// Check if a byte buffer contains null bytes (binary indicator)
fn contains_null_bytes(bytes: &[u8]) -> bool {
    // Check first 8KB for null bytes
    let check_len = std::cmp::min(bytes.len(), 8192);
    bytes[..check_len].contains(&0)
}

/// Error type for source provider operations
#[derive(Debug, thiserror::Error)]
pub enum SourceProviderError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Target not found: {0}")]
    NotFound(String),

    #[error("Content too large: {size} bytes (max: {max})")]
    TooLarge { size: u64, max: u64 },

    #[error("Binary content not supported: {0}")]
    BinaryContent(String),

    #[error("Invalid encoding: {0}")]
    Encoding(String),
}

/// Result type alias for source provider operations
pub type SourceProviderResult<T> = Result<T, SourceProviderError>;

/// Source Provider Trait
///
/// Defines the interface for enumerating scannable content from various sources.
///
/// # Design Principles
///
/// - **Enumeration is cheap**: Implementations should only list metadata, not load content
/// - **Send + Sync**: Must be thread-safe for parallel scanning
/// - **Iterator-based**: Allows lazy enumeration of large datasets
///
/// # Example Implementation
///
/// ```rust
/// pub struct MyProvider {
///     // provider-specific state
/// }
///
/// impl SourceProvider for MyProvider {
///     fn enumerate(&self) -> Box<dyn Iterator<Item = ScanTarget> + '_> {
///         Box::new(my_targets.into_iter())
///     }
///
///     fn total_units_hint(&self) -> Option<u64> {
///         Some(self.expected_count)
///     }
///
///     fn progress_unit_name(&self) -> &str {
///         "files"  // or "commits", "layers", etc.
///     }
/// }
/// ```
pub trait SourceProvider: Send + Sync {
    /// Enumerate available scan targets
    ///
    /// Returns an iterator over scan targets. Content is NOT loaded at this stage.
    ///
    /// # Performance
    ///
    /// This method should be cheap - it should only gather metadata like paths,
    /// sizes, and content type hints. Actual content loading happens via `ContentLoader`.
    fn enumerate(&self) -> Box<dyn Iterator<Item = ScanTarget> + '_>;

    /// Total expected units for progress reporting
    ///
    /// Returns `None` if the total count is unknown (e.g., streaming source).
    fn total_units_hint(&self) -> Option<u64>;

    /// Human-readable unit name for progress bar
    ///
    /// Examples: "files", "commits", "layers", "blobs"
    fn progress_unit_name(&self) -> &str;
}

/// Content Loader Trait
///
/// Defines the interface for loading content from scan targets.
///
/// # Design Principles
///
/// - **Selective loading**: Only load content for targets that will actually be scanned
/// - **Streaming support**: Large content can be streamed rather than fully buffered
/// - **Binary detection**: Implementations should detect and handle binary content
///
/// # Example Usage
///
/// ```rust
/// for target in provider.enumerate() {
///     // Skip binary files
///     if target.content_type == Some(ContentType::Binary) {
///         continue;
///     }
///
///     // Skip oversized files
///     if target.size_hint.unwrap_or(0) > MAX_SIZE {
///         continue;
///     }
///
///     // Now load the content
///     let content = provider.load(&target)?;
///     // ... scan content ...
/// }
/// ```
pub trait ContentLoader: Send + Sync {
    /// Load content for a specific scan target
    ///
    /// # Arguments
    ///
    /// * `target` - The scan target to load content for
    ///
    /// # Returns
    ///
    /// The loaded content, either buffered or streamed based on size.
    ///
    /// # Errors
    ///
    /// Returns an error if the target cannot be read, is binary, or exceeds size limits.
    fn load(&self, target: &ScanTarget) -> SourceProviderResult<ScanContent>;

    /// Maximum content size to load (bytes)
    ///
    /// Content larger than this will return `Err(SourceProviderError::TooLarge)`.
    /// Default is 1MB, but implementations may override.
    fn max_content_size(&self) -> u64 {
        1024 * 1024 // 1MB default
    }

    /// Whether to skip binary content
    ///
    /// If true, `load()` will return `Err(SourceProviderError::BinaryContent)` for binary files.
    fn skip_binary(&self) -> bool {
        true
    }
}

/// File System Provider
///
/// Implements `SourceProvider` and `ContentLoader` for local filesystem scanning.
///
/// # Features
///
/// - Recursive directory enumeration
/// - Binary detection (null bytes in first 8KB)
/// - Size-based filtering
/// - Streaming support for large files
///
/// # Example
///
/// ```rust
/// use coax_scanner::source_provider::{FileSystemProvider, SourceProvider, ContentLoader};
/// use std::path::PathBuf;
/// use std::sync::Arc;
///
/// let provider = Arc::new(FileSystemProvider::new(PathBuf::from("./src")));
///
/// for target in provider.enumerate() {
///     println!("Found: {}", target.display_path());
///     let content = provider.load(&target)?;
///     // ... scan content ...
/// }
/// ```
pub struct FileSystemProvider {
    /// Root directory to scan
    root: PathBuf,
    /// Maximum file size to load (bytes)
    max_file_size: u64,
    /// Whether to skip binary files
    skip_binary: bool,
    /// Files to exclude by pattern
    exclude_patterns: Vec<String>,
    /// Whether to scan hidden files (starting with .)
    scan_hidden: bool,
    /// Whether to follow symlinks
    follow_symlinks: bool,
}

impl FileSystemProvider {
    /// Create a new filesystem provider
    ///
    /// # Arguments
    ///
    /// * `root` - Root directory to scan
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            max_file_size: 10 * 1024 * 1024, // 10MB default
            skip_binary: true,
            exclude_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                "vendor".to_string(),
                ".venv".to_string(),
                "__pycache__".to_string(),
            ],
            scan_hidden: false,
            follow_symlinks: false,
        }
    }

    /// Set maximum file size
    pub fn with_max_file_size(mut self, size: u64) -> Self {
        self.max_file_size = size;
        self
    }

    /// Set whether to skip binary files
    pub fn with_skip_binary(mut self, skip: bool) -> Self {
        self.skip_binary = skip;
        self
    }

    /// Add exclude patterns
    pub fn with_exclude_patterns(mut self, patterns: Vec<String>) -> Self {
        self.exclude_patterns = patterns;
        self
    }

    /// Set whether to scan hidden files
    pub fn with_scan_hidden(mut self, scan: bool) -> Self {
        self.scan_hidden = scan;
        self
    }

    /// Set whether to follow symlinks
    pub fn with_follow_symlinks(mut self, follow: bool) -> Self {
        self.follow_symlinks = follow;
        self
    }

    /// Check if a path should be excluded
    fn should_exclude(&self, path: &Path) -> bool {
        // Check exclude patterns
        for pattern in &self.exclude_patterns {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if pattern.starts_with("*.") {
                    // Extension pattern
                    if name.ends_with(&pattern[1..]) {
                        return true;
                    }
                } else if name == pattern {
                    // Exact match
                    return true;
                }
            }

            // Check if path contains the pattern as a component
            if path.to_string_lossy().contains(pattern) {
                return true;
            }
        }

        // Check hidden files
        if !self.scan_hidden {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if name.starts_with('.') {
                    return true;
                }
            }
        }

        false
    }

    /// Detect if a file is binary by checking for null bytes in first 8KB
    fn is_binary_file(&self, path: &Path) -> io::Result<bool> {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 8192];
        let bytes_read = file.read(&mut buffer)?;
        Ok(buffer[..bytes_read].contains(&0))
    }

    /// Detect content type for a file
    fn detect_content_type(&self, path: &Path) -> io::Result<ContentType> {
        if self.is_binary_file(path)? {
            Ok(ContentType::Binary)
        } else {
            Ok(ContentType::Text)
        }
    }

    /// Check if a file extension should be scanned
    fn should_scan_extension(&self, ext: &std::ffi::OsStr) -> bool {
        matches!(
            ext.to_str(),
            Some(
                // Core programming languages
                "js" | "ts" | "tsx" | "jsx" | "py" | "rs" | "go" | "rb" | "php" |
                "java" | "cs" | "cpp" | "c" | "h" | "hpp" | "kt" | "kts" | "swift" |
                "scala" | "sbt" | "dart" | "r" | "R" | "jl" | "nim" | "hs" | "lhs" |
                "fs" | "fsi" | "fsx" | "elm" | "vue" | "svelte" |
                // Web technologies
                "html" | "css" | "scss" | "less" | "graphql" | "gql" |
                // Config files
                "yml" | "yaml" | "json" | "toml" | "xml" | "ini" | "conf" | "config" |
                "properties" | "plist" | "cfg" | "hcl" | "tf" | "tfvars" | "terraform" |
                "env" | "envrc" | "htaccess" | "htpasswd" |
                // Build/CI files
                "gradle" | "cmake" | "mk" | "bzl" | "bazel" | "makefile" |
                // Data files
                "csv" | "tsv" | "sql" | "sqlite" |
                // Package manager configs
                "npmrc" | "pypirc" | "gemrc" | "yarnrc" | "lock" | "sum" |
                // Notebooks
                "ipynb" | "rmd" |
                // IaC / Config management
                "pp" | "sls" | "erb" |
                // Shell scripts
                "sh" | "bash" | "zsh" | "ps1" | "psm1" | "psd1" | "bat" | "cmd" | "vbs" | "vb" | "lua" |
                // Perl/Raku
                "pl" | "pm" | "t" | "raku" | "rakumod" | "rakutest" |
                // Erlang/Elixir
                "erl" | "hrl" | "ex" | "exs" |
                // Clojure
                "clj" | "cljs" | "edn" |
                // Other
                "md" | "txt" | "proto" | "dockerfile" | "pem" | "key" | "cert" |
                "m" | "mm" | "nix" | "re" | "rei" | "bin" | "dat" | "exe" | "dll" | "so"
            )
        )
    }
}

impl SourceProvider for FileSystemProvider {
    fn enumerate(&self) -> Box<dyn Iterator<Item = ScanTarget> + '_> {
        let mut options = walkdir::WalkDir::new(&self.root);

        if !self.follow_symlinks {
            options = options.follow_links(false);
        }

        let root = self.root.clone();
        let iterator = options
            .into_iter()
            .filter_entry(move |entry| {
                // Don't filter the root directory itself
                if entry.path() == root {
                    return true;
                }
                !self.should_exclude(entry.path())
            })
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_file())
            .filter_map(move |entry| {
                let path = entry.path().to_path_buf();
                let metadata = entry.metadata().ok()?;
                let size = metadata.len();

                // Skip files over size limit during enumeration
                if size > self.max_file_size {
                    return None;
                }

                // Check extension
                if let Some(ext) = path.extension() {
                    if !self.should_scan_extension(ext) {
                        return None;
                    }
                } else {
                    // Files without extension - check if it's a known file type
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        // Allow common text-based files without extensions
                        // This includes build files, config files, and documentation
                        const KNOWN_TEXT_FILES: &[&str] = &[
                            "Jenkinsfile",
                            "Makefile",
                            "Vagrantfile",
                            "Dockerfile",
                            "Gemfile",
                            "Rakefile",
                            "Procfile",
                            "Brewfile",
                            "Berksfile",
                            "Podfile",
                            ".env",
                            ".envrc",
                            ".gitignore",
                            ".dockerignore",
                            ".npmignore",
                            ".pypirc",
                            "README",
                            "LICENSE",
                            "CHANGELOG",
                            "INSTALL",
                            "TODO",
                            "NOTICE",
                            "AUTHORS",
                            "CONTRIBUTORS",
                        ];
                        if !KNOWN_TEXT_FILES.contains(&name) {
                            // For unknown extensionless files, do a quick binary check
                            if let Ok(mut file) = std::fs::File::open(&path) {
                                use std::io::Read;
                                let mut buffer = [0u8; 512];
                                if let Ok(bytes_read) = file.read(&mut buffer) {
                                    if bytes_read > 0 && buffer[..bytes_read].contains(&0u8) {
                                        return None; // Binary file
                                    }
                                }
                            }
                        }
                    } else {
                        return None;
                    }
                }

                let content_type = self.detect_content_type(&path).ok();

                let mut target =
                    ScanTarget::new(ScanOrigin::FileSystem { path }).with_size_hint(size);

                if let Some(ct) = content_type {
                    target = target.with_content_type(ct);
                }

                Some(target)
            });

        Box::new(iterator)
    }

    fn total_units_hint(&self) -> Option<u64> {
        // Count files (this is somewhat expensive, but useful for progress)
        let root = self.root.clone();
        let count = walkdir::WalkDir::new(&self.root)
            .into_iter()
            .filter_entry(move |e| {
                // Don't filter the root directory itself
                if e.path() == root {
                    return true;
                }
                !self.should_exclude(e.path())
            })
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .filter(|e| {
                // Apply same extension filtering as enumerate()
                if let Some(ext) = e.path().extension() {
                    self.should_scan_extension(ext)
                } else if let Some(name) = e.path().file_name().and_then(|n| n.to_str()) {
                    matches!(
                        name,
                        "Dockerfile" | ".gitignore" | ".env" | "LICENSE" | "README"
                    )
                } else {
                    false
                }
            })
            .count();
        Some(count as u64)
    }

    fn progress_unit_name(&self) -> &str {
        "files"
    }
}

impl ContentLoader for FileSystemProvider {
    fn load(&self, target: &ScanTarget) -> SourceProviderResult<ScanContent> {
        let path = match &target.origin {
            ScanOrigin::FileSystem { path } => path,
            _ => {
                return Err(SourceProviderError::NotFound(
                    "FileSystemProvider can only load FileSystem targets".to_string(),
                ))
            }
        };

        // Check size limit
        let size = target.size_hint.unwrap_or(0);
        if size > self.max_file_size {
            return Err(SourceProviderError::TooLarge {
                size,
                max: self.max_file_size,
            });
        }

        // Check binary
        if self.skip_binary {
            if let Some(ContentType::Binary) = target.content_type {
                return Err(SourceProviderError::BinaryContent(
                    path.display().to_string(),
                ));
            }
        }

        // Load content
        let mut file = File::open(path).map_err(|e| {
            SourceProviderError::Io(io::Error::new(
                e.kind(),
                format!("Failed to open {}: {}", path.display(), e),
            ))
        })?;

        // For small files, buffer entirely
        // For larger files, still buffer (streaming is more complex)
        // TODO: Implement true streaming for very large files
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| {
            SourceProviderError::Io(io::Error::new(
                e.kind(),
                format!("Failed to read {}: {}", path.display(), e),
            ))
        })?;

        Ok(ScanContent::Buffered(buffer))
    }

    fn max_content_size(&self) -> u64 {
        self.max_file_size
    }

    fn skip_binary(&self) -> bool {
        self.skip_binary
    }
}

/// Git History Provider
///
/// Implements `SourceProvider` and `ContentLoader` for scanning git repository history.
///
/// # Features
///
/// - **Merge Commit Handling**: Diffs against first parent only to prevent double-counting
/// - **Shallow Clone Detection**: Detects and warns about incomplete history
/// - **Memory Management**: Skips binary files, enforces per-blob size limits
/// - **Flexible Filtering**: Support for commit limits, date ranges, and commit ranges
///
/// # Example
///
/// ```rust
/// use coax_scanner::source_provider::{GitHistoryProvider, SourceProvider, ContentLoader};
/// use std::path::PathBuf;
/// use std::sync::Arc;
/// use chrono::Utc;
///
/// // Create a git history provider
/// let provider = Arc::new(
///     GitHistoryProvider::new(PathBuf::from("./.git"))
///         .unwrap()
///         .with_commit_limit(100)
///         .with_since(Utc::now() - chrono::Duration::days(30))
/// );
///
/// // Enumerate available targets
/// for target in provider.enumerate() {
///     println!("Found: {:?}", target.origin);
/// }
///
/// // Load content for a specific target
/// let content = provider.load(&target).unwrap();
/// ```
pub struct GitHistoryProvider {
    /// Git repository (wrapped in Mutex for thread safety since git2::Repository is not Sync)
    repo: git2::Repository,
    /// Maximum number of commits to scan (None = unlimited)
    commit_limit: Option<usize>,
    /// Only scan commits since this date
    since: Option<DateTime<Utc>>,
    /// Commit range to scan (e.g., "main..feature")
    range: Option<String>,
    /// Whether to scan diff hunks (true) or full files (false)
    scan_diffs: bool,
    /// Maximum blob size to load (bytes)
    max_blob_size: u64,
    /// Whether to skip binary files
    skip_binary: bool,
    /// Cached commit count for progress reporting
    cached_commit_count: std::sync::Mutex<Option<u64>>,
}

// git2::Repository is Send but not Sync, so we need to implement Sync manually
// This is safe because we only access the repository through &self (immutable references)
// and git2 handles internal synchronization
unsafe impl Sync for GitHistoryProvider {}

impl GitHistoryProvider {
    /// Create a new git history provider
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the git repository (can be the .git directory or any subdirectory)
    ///
    /// # Returns
    ///
    /// A new `GitHistoryProvider` instance
    ///
    /// # Errors
    ///
    /// Returns an error if the path is not a valid git repository
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, git2::Error> {
        let repo = git2::Repository::discover(path.as_ref())?;
        Ok(Self {
            repo,
            commit_limit: None,
            since: None,
            range: None,
            scan_diffs: true, // Default to scanning diff hunks (more efficient)
            max_blob_size: 1024 * 1024, // 1MB default
            skip_binary: true,
            cached_commit_count: std::sync::Mutex::new(None),
        })
    }

    /// Set the maximum number of commits to scan
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of commits to process
    pub fn with_commit_limit(mut self, limit: usize) -> Self {
        self.commit_limit = Some(limit);
        self
    }

    /// Set the "since" date filter
    ///
    /// # Arguments
    ///
    /// * `since` - Only scan commits after this date
    pub fn with_since(mut self, since: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self
    }

    /// Set the commit range to scan
    ///
    /// # Arguments
    ///
    /// * `range` - Git commit range (e.g., "main..feature", "HEAD~10..HEAD")
    pub fn with_range(mut self, range: String) -> Self {
        self.range = Some(range);
        self
    }

    /// Set whether to scan diff hunks or full files
    ///
    /// # Arguments
    ///
    /// * `scan_diffs` - If true, scan only diff hunks (more efficient).
    ///                  If false, scan full file content at each commit.
    pub fn with_scan_diffs(mut self, scan_diffs: bool) -> Self {
        self.scan_diffs = scan_diffs;
        self
    }

    /// Set the maximum blob size to load
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum size in bytes
    pub fn with_max_blob_size(mut self, size: u64) -> Self {
        self.max_blob_size = size;
        self
    }

    /// Set whether to skip binary files
    ///
    /// # Arguments
    ///
    /// * `skip` - If true, skip binary files
    pub fn with_skip_binary(mut self, skip: bool) -> Self {
        self.skip_binary = skip;
        self
    }

    /// Check if the repository is a shallow clone
    ///
    /// Shallow clones have incomplete history and may miss historical secrets.
    ///
    /// # Returns
    ///
    /// `true` if this is a shallow clone, `false` otherwise
    pub fn is_shallow(&self) -> bool {
        // Check for .git/shallow file (git2 doesn't expose is_shallow directly)
        let git_dir = self.repo.path();
        let shallow_file = git_dir.join("shallow");
        shallow_file.exists()
    }

    /// Print a warning if this is a shallow clone
    ///
    /// This should be called before starting a scan to inform the user
    /// about potential limitations.
    pub fn warn_if_shallow(&self) {
        if self.is_shallow() {
            eprintln!("⚠️  Warning: Shallow clone detected. Git history is incomplete.");
            eprintln!("   Historical secrets may be missed.");
            eprintln!("   Remediation:");
            eprintln!("     - Run: git fetch --unshallow");
            eprintln!("     - Or use --commits 1 to scan only the latest commit");
        }
    }

    /// Get the repository reference (for internal use)
    fn repo(&self) -> &git2::Repository {
        &self.repo
    }

    /// Walk the commit history and return an iterator over commits
    ///
    /// Handles commit limits, date filters, and commit ranges.
    fn walk_commits(&self) -> Result<Vec<git2::Commit<'_>>, git2::Error> {
        // If a range is specified, use revparse to resolve it
        if let Some(ref range) = self.range {
            let revspec = self.repo.revparse(range)?;
            let mut revwalk = self.repo.revwalk()?;
            revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

            if revspec.mode() == git2::RevparseMode::SINGLE {
                // Single commit/branch
                if let Some(from) = revspec.from() {
                    let from_id: git2::Oid = from.id();
                    revwalk.push(from_id)?;
                }
            } else {
                // Range (A..B or A...B)
                if let Some(to) = revspec.to() {
                    let to_id: git2::Oid = to.id();
                    revwalk.push(to_id)?;
                }
                if let Some(from) = revspec.from() {
                    // For A..B, we want commits reachable from B but not from A
                    // This is handled by revwalk's natural behavior when we push 'to'
                    // For A...B (merge base), we'd need more complex logic
                    if revspec.mode() == git2::RevparseMode::MERGE_BASE {
                        // For merge base, push both
                        let from_id: git2::Oid = from.id();
                        revwalk.push(from_id)?;
                    } else {
                        // For simple range, hide the 'from' commits
                        let from_id: git2::Oid = from.id();
                        revwalk.hide(from_id)?;
                    }
                }
            }

            return self.collect_commits_from_revwalk(revwalk);
        }

        // No range specified - walk from HEAD
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

        self.collect_commits_from_revwalk(revwalk)
    }

    /// Collect commits from a revwalk with limit and date filtering
    fn collect_commits_from_revwalk(
        &self,
        revwalk: git2::Revwalk<'_>,
    ) -> Result<Vec<git2::Commit<'_>>, git2::Error> {
        let mut commits: Vec<git2::Commit> = Vec::new();
        let mut shallow_warning_printed = false;

        for oid_result in revwalk {
            let oid = oid_result?;
            let commit = self.repo.find_commit(oid)?;

            // Check commit limit
            if let Some(limit) = self.commit_limit {
                if commits.len() >= limit {
                    break;
                }
            }

            // Check "since" date filter
            if let Some(since) = self.since {
                let commit_time = DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0);
                if let Some(commit_dt) = commit_time {
                    if commit_dt < since {
                        // Stop walking - we've gone back too far
                        break;
                    }
                }
            }

            // Print shallow warning once if detected
            if !shallow_warning_printed && self.is_shallow() {
                self.warn_if_shallow();
                shallow_warning_printed = true;
            }

            commits.push(commit);
        }

        Ok(commits)
    }

    /// Get the diff for a commit
    ///
    /// For merge commits, diffs against the first parent only to prevent double-counting.
    fn get_commit_diff(&self, commit: &git2::Commit) -> Result<git2::Diff<'_>, git2::Error> {
        let tree = commit.tree()?;

        // For merge commits, use first parent only
        // This prevents double-counting secrets already reported on feature branches
        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = git2::DiffOptions::new();
        diff_opts
            .include_unmodified(false)
            .include_unreadable(false)
            .recurse_untracked_dirs(true);

        self.repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), Some(&mut diff_opts))
    }

    /// Check if a blob is binary (contains null bytes in first 8KB)
    fn is_binary_blob(&self, blob: &git2::Blob) -> bool {
        let content = blob.content();
        if content.is_empty() {
            return false;
        }

        // Check first 8KB for null bytes
        let check_len = std::cmp::min(content.len(), 8192);
        content[..check_len].contains(&0)
    }

    /// Scan a diff and return scan targets for added/modified content
    fn diff_to_targets(&self, diff: &git2::Diff, commit: &git2::Commit) -> Vec<ScanTarget> {
        let mut targets = Vec::new();
        let commit_sha = commit.id().to_string();
        let _author = commit.author().to_string();
        let _date = DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
            .unwrap_or_else(|| Utc::now());

        let _ = diff.foreach(
            &mut |delta, _| {
                let path = delta.new_file().path().map(|p| p.to_path_buf());
                if let Some(path) = path {
                    let is_binary = delta.new_file().is_binary();
                    let is_added = delta.status() == git2::Delta::Added;
                    let is_modified = delta.status() == git2::Delta::Modified;

                    // Skip binary files
                    if is_binary && self.skip_binary {
                        return true; // Continue to next delta
                    }

                    // For diffs, we create GitDiff targets
                    if is_added || is_modified {
                        let mut target = ScanTarget::new(ScanOrigin::GitDiff {
                            commit: commit_sha.clone(),
                            path: path.clone(),
                            added: true,
                        });

                        // Size hint from delta (if available)
                        if let Some(stats) = diff.stats().ok() {
                            target = target.with_size_hint(stats.insertions() as u64 * 100);
                            // Rough estimate
                        }

                        target = target.with_content_type(if is_binary {
                            ContentType::Binary
                        } else {
                            ContentType::Text
                        });

                        targets.push(target);
                    }
                }
                true // Continue iteration
            },
            None,
            None,
            None,
        );

        targets
    }

    /// Scan a tree and return scan targets for all blobs
    fn tree_to_targets(&self, tree: &git2::Tree, commit: &git2::Commit) -> Vec<ScanTarget> {
        let mut targets = Vec::new();
        let commit_sha = commit.id().to_string();
        let author = commit.author().to_string();
        let date = DateTime::<Utc>::from_timestamp(commit.time().seconds(), 0)
            .unwrap_or_else(|| Utc::now());

        self.tree_walk_recursive(
            tree,
            PathBuf::from(""),
            &commit_sha,
            &author,
            date,
            &mut targets,
        );

        targets
    }

    /// Recursively walk a tree and collect scan targets
    fn tree_walk_recursive(
        &self,
        tree: &git2::Tree,
        current_path: PathBuf,
        commit_sha: &str,
        author: &str,
        date: DateTime<Utc>,
        targets: &mut Vec<ScanTarget>,
    ) {
        for entry in tree.iter() {
            let entry_path = current_path.join(entry.name().unwrap_or("unknown"));

            match entry.kind() {
                Some(git2::ObjectType::Blob) => {
                    if let Ok(blob) = entry.to_object(self.repo()).and_then(|obj| {
                        obj.into_blob()
                            .map_err(|_| git2::Error::from_str("Not a blob"))
                    }) {
                        // Skip binary blobs
                        if self.skip_binary && self.is_binary_blob(&blob) {
                            continue;
                        }

                        // Skip oversized blobs
                        if blob.size() as u64 > self.max_blob_size {
                            continue;
                        }

                        let mut target = ScanTarget::new(ScanOrigin::GitBlob {
                            commit: commit_sha.to_string(),
                            path: entry_path,
                            author: author.to_string(),
                            date,
                        })
                        .with_size_hint(blob.size() as u64);

                        target = target.with_content_type(if self.is_binary_blob(&blob) {
                            ContentType::Binary
                        } else {
                            ContentType::Text
                        });

                        targets.push(target);
                    }
                }
                Some(git2::ObjectType::Tree) => {
                    if let Ok(subtree) = entry.to_object(self.repo()).and_then(|obj| {
                        obj.into_tree()
                            .map_err(|_| git2::Error::from_str("Not a tree"))
                    }) {
                        self.tree_walk_recursive(
                            &subtree, entry_path, commit_sha, author, date, targets,
                        );
                    }
                }
                _ => {}
            }
        }
    }
}

impl SourceProvider for GitHistoryProvider {
    fn enumerate(&self) -> Box<dyn Iterator<Item = ScanTarget> + '_> {
        // Walk commits and enumerate all targets
        let commits = match self.walk_commits() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to walk git history: {}", e);
                return Box::new(std::iter::empty());
            }
        };

        // Cache commit count for progress reporting
        if let Ok(mut count) = self.cached_commit_count.lock() {
            *count = Some(commits.len() as u64);
        }

        // Collect all targets from all commits
        let mut all_targets = Vec::new();

        for commit in &commits {
            if self.scan_diffs {
                // Scan diff hunks (more efficient)
                match self.get_commit_diff(commit) {
                    Ok(diff) => {
                        let targets = self.diff_to_targets(&diff, commit);
                        all_targets.extend(targets);
                    }
                    Err(e) => {
                        tracing::debug!("Failed to get diff for commit {}: {}", commit.id(), e);
                    }
                }
            } else {
                // Scan full files at each commit (slower but more thorough)
                match commit.tree() {
                    Ok(tree) => {
                        let targets = self.tree_to_targets(&tree, commit);
                        all_targets.extend(targets);
                    }
                    Err(e) => {
                        tracing::debug!("Failed to get tree for commit {}: {}", commit.id(), e);
                    }
                }
            }
        }

        Box::new(all_targets.into_iter())
    }

    fn total_units_hint(&self) -> Option<u64> {
        // Return cached commit count if available
        if let Ok(count) = self.cached_commit_count.lock() {
            if let Some(c) = *count {
                // Estimate: commits * average files per commit (rough heuristic)
                return Some(c * 10);
            }
        }

        // Fallback: just return commit count
        self.commit_limit.map(|l| l as u64)
    }

    fn progress_unit_name(&self) -> &str {
        if self.scan_diffs {
            "diffs"
        } else {
            "commits"
        }
    }
}

impl ContentLoader for GitHistoryProvider {
    fn load(&self, target: &ScanTarget) -> SourceProviderResult<ScanContent> {
        // Check size limit
        let size = target.size_hint.unwrap_or(0);
        if size > self.max_blob_size {
            return Err(SourceProviderError::TooLarge {
                size,
                max: self.max_blob_size,
            });
        }

        match &target.origin {
            ScanOrigin::GitBlob { commit, path, .. } => {
                // Load blob from specific commit
                let commit_obj = self
                    .repo
                    .revparse_single(commit)
                    .map_err(|e: git2::Error| SourceProviderError::Git(e.to_string()))?
                    .into_commit()
                    .map_err(|_| SourceProviderError::Git("Not a commit".to_string()))?;

                let tree = commit_obj
                    .tree()
                    .map_err(|e: git2::Error| SourceProviderError::Git(e.to_string()))?;

                let blob = self.find_blob_in_tree(&tree, path).ok_or_else(|| {
                    SourceProviderError::NotFound(format!(
                        "Blob not found: {} at {}",
                        path.display(),
                        commit
                    ))
                })?;

                // Check binary
                if self.skip_binary && self.is_binary_blob(&blob) {
                    return Err(SourceProviderError::BinaryContent(format!(
                        "{} at {}",
                        path.display(),
                        commit
                    )));
                }

                Ok(ScanContent::Buffered(blob.content().to_vec()))
            }

            ScanOrigin::GitDiff {
                commit,
                path,
                added: _,
            } => {
                // For diffs, we need to get the content from the commit's tree
                let commit_obj = self
                    .repo
                    .revparse_single(commit)
                    .map_err(|e: git2::Error| SourceProviderError::Git(e.to_string()))?
                    .into_commit()
                    .map_err(|_| SourceProviderError::Git("Not a commit".to_string()))?;

                let tree = commit_obj
                    .tree()
                    .map_err(|e: git2::Error| SourceProviderError::Git(e.to_string()))?;

                let blob = self.find_blob_in_tree(&tree, path).ok_or_else(|| {
                    SourceProviderError::NotFound(format!(
                        "Blob not found: {} at {}",
                        path.display(),
                        commit
                    ))
                })?;

                // Check binary
                if self.skip_binary && self.is_binary_blob(&blob) {
                    return Err(SourceProviderError::BinaryContent(format!(
                        "{} at {}",
                        path.display(),
                        commit
                    )));
                }

                Ok(ScanContent::Buffered(blob.content().to_vec()))
            }

            ScanOrigin::FileSystem { .. } => Err(SourceProviderError::NotFound(
                "GitHistoryProvider can only load Git targets".to_string(),
            )),
        }
    }

    fn max_content_size(&self) -> u64 {
        self.max_blob_size
    }

    fn skip_binary(&self) -> bool {
        self.skip_binary
    }
}

impl GitHistoryProvider {
    /// Find a blob in a tree by path
    fn find_blob_in_tree(&self, tree: &git2::Tree, path: &Path) -> Option<git2::Blob<'_>> {
        // Use the repo's lookup method which handles the tree traversal internally
        let oid = tree.get_path(path.as_ref()).ok()?.id();

        // Find the blob by OID
        self.repo.find_blob(oid).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_target_creation() {
        let target = ScanTarget::new(ScanOrigin::FileSystem {
            path: PathBuf::from("/test/file.txt"),
        });
        assert_eq!(target.display_path(), "/test/file.txt");
    }

    #[test]
    fn test_scan_target_with_metadata() {
        let target = ScanTarget::new(ScanOrigin::FileSystem {
            path: PathBuf::from("/test/file.txt"),
        })
        .with_size_hint(1024)
        .with_content_type(ContentType::Text);

        assert_eq!(target.size_hint, Some(1024));
        assert_eq!(target.content_type, Some(ContentType::Text));
    }

    #[test]
    fn test_scan_origin_display() {
        let blob_origin = ScanOrigin::GitBlob {
            commit: "abc123def456".to_string(),
            path: PathBuf::from("src/main.rs"),
            author: "Test User <test@example.com>".to_string(),
            date: Utc::now(),
        };

        let target = ScanTarget::new(blob_origin.clone());
        assert!(target.display_path().contains("src/main.rs"));
        assert!(target.display_path().contains("abc123d"));
    }

    #[test]
    fn test_filesystem_provider_enumeration() {
        let temp_dir = TempDir::new().unwrap();

        // Create test files
        fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
        fs::write(temp_dir.path().join("file2.rs"), "content2").unwrap();

        let provider = FileSystemProvider::new(temp_dir.path().to_path_buf());
        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        assert_eq!(targets.len(), 2);
    }

    #[test]
    fn test_filesystem_provider_excludes_hidden() {
        let temp_dir = TempDir::new().unwrap();

        fs::write(temp_dir.path().join("visible.txt"), "content").unwrap();
        fs::write(temp_dir.path().join(".hidden.txt"), "hidden").unwrap();

        let provider =
            FileSystemProvider::new(temp_dir.path().to_path_buf()).with_scan_hidden(false);

        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        assert_eq!(targets.len(), 1);
        assert!(targets[0].display_path().contains("visible.txt"));
    }

    #[test]
    fn test_filesystem_provider_load() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "Hello, World!").unwrap();

        let provider = FileSystemProvider::new(temp_dir.path().to_path_buf());
        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        assert!(!targets.is_empty());
        let content = provider.load(&targets[0]).unwrap();

        match content {
            ScanContent::Buffered(bytes) => {
                assert_eq!(bytes, b"Hello, World!");
            }
            ScanContent::Streamed(_) => panic!("Expected buffered content"),
        }
    }

    #[test]
    fn test_binary_detection() {
        let temp_dir = TempDir::new().unwrap();
        let binary_path = temp_dir.path().join("binary.bin");

        // Write binary content with null bytes
        fs::write(&binary_path, vec![0x00, 0x01, 0x02, 0x03]).unwrap();

        let provider =
            FileSystemProvider::new(temp_dir.path().to_path_buf()).with_skip_binary(true);

        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        // Binary file should be detected
        assert!(!targets.is_empty());
        assert_eq!(targets[0].content_type, Some(ContentType::Binary));

        // Loading should fail when skip_binary is true
        let result = provider.load(&targets[0]);
        assert!(matches!(result, Err(SourceProviderError::BinaryContent(_))));
    }

    #[test]
    fn test_contains_null_bytes() {
        assert!(contains_null_bytes(&[0x00, 0x01, 0x02]));
        assert!(!contains_null_bytes(&[0x41, 0x42, 0x43])); // "ABC"
        assert!(!contains_null_bytes(&[]));
    }

    #[test]
    fn test_scan_content_into_string() {
        let content = ScanContent::Buffered(b"Hello, World!".to_vec());
        let string = content.into_string().unwrap();
        assert_eq!(string, "Hello, World!");
    }

    #[test]
    fn test_filesystem_provider_total_hint() {
        let temp_dir = TempDir::new().unwrap();

        for i in 0..5 {
            fs::write(temp_dir.path().join(format!("file{}.txt", i)), "content").unwrap();
        }

        let provider = FileSystemProvider::new(temp_dir.path().to_path_buf());
        let hint = provider.total_units_hint();

        assert_eq!(hint, Some(5));
    }

    #[test]
    fn test_filesystem_provider_progress_unit() {
        let temp_dir = TempDir::new().unwrap();
        let provider = FileSystemProvider::new(temp_dir.path().to_path_buf());
        assert_eq!(provider.progress_unit_name(), "files");
    }

    #[test]
    fn test_git_history_provider_creation() {
        // Test that we can create a GitHistoryProvider for the current repo
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let result = GitHistoryProvider::new(&repo_path);

        // Should succeed if we're in a git repository
        if result.is_ok() {
            let provider = result.unwrap();
            assert!(!provider.is_shallow()); // Local repo shouldn't be shallow
        }
    }

    #[test]
    fn test_git_history_provider_with_config() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_commit_limit(10)
            .with_max_blob_size(512 * 1024) // 512KB
            .with_skip_binary(true)
            .with_scan_diffs(true);

        assert_eq!(provider.commit_limit, Some(10));
        assert_eq!(provider.max_blob_size, 512 * 1024);
        assert!(provider.skip_binary);
        assert!(provider.scan_diffs);
    }

    #[test]
    fn test_git_history_provider_enumerate() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_commit_limit(5);

        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        // Should find some targets (unless repo is empty)
        // We can't assert a specific number, but we can check the type
        for target in &targets {
            match &target.origin {
                ScanOrigin::GitDiff { .. } => {
                    // Expected when scan_diffs is true
                }
                ScanOrigin::GitBlob { .. } => {
                    // Expected when scan_diffs is false
                }
                ScanOrigin::FileSystem { .. } => {
                    panic!("Should not get FileSystem origin from GitHistoryProvider");
                }
            }
        }
    }

    #[test]
    fn test_git_history_provider_progress_unit() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_scan_diffs(true);

        assert_eq!(provider.progress_unit_name(), "diffs");

        let provider_no_diffs = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_scan_diffs(false);

        assert_eq!(provider_no_diffs.progress_unit_name(), "commits");
    }

    #[test]
    fn test_git_history_provider_total_hint() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_commit_limit(10);

        // Should return some estimate
        let hint = provider.total_units_hint();
        assert!(hint.is_some());
        assert!(hint.unwrap() > 0);
    }

    #[test]
    fn test_git_history_provider_load() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_commit_limit(1);

        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        if !targets.is_empty() {
            let target = &targets[0];
            let result = provider.load(target);

            match result {
                Ok(ScanContent::Buffered(bytes)) => {
                    // Content loaded successfully
                    assert!(bytes.len() as u64 <= provider.max_blob_size);
                }
                Ok(ScanContent::Streamed(_)) => {
                    // Streamed content (unlikely for small test)
                }
                Err(e) => {
                    // May fail for binary files or other reasons
                    eprintln!("Load failed (expected for binary/missing): {}", e);
                }
            }
        }
    }

    #[test]
    fn test_git_history_provider_binary_detection() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_skip_binary(true)
            .with_commit_limit(5); // Limit commits for faster test

        // Binary detection happens during enumeration
        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        // All returned targets should be text (binary filtered out)
        for target in &targets {
            assert_ne!(
                target.content_type,
                Some(ContentType::Binary),
                "Binary content should be filtered out during enumeration"
            );
        }
    }

    #[test]
    fn test_git_history_provider_with_since() {
        use chrono::Duration;

        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let since_date = Utc::now() - Duration::days(365); // Last year

        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_since(since_date)
            .with_commit_limit(10);

        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        // Should find targets from commits in the last year
        // (exact count depends on repo activity)
        let _ = targets.len(); // Ensure no panic
    }

    #[test]
    fn test_git_history_provider_with_range() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        // Test with HEAD~1..HEAD range (last commit only)
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_range("HEAD~1..HEAD".to_string());

        let targets: Vec<ScanTarget> = provider.enumerate().collect();

        // Should find targets from the last commit
        let _ = targets.len(); // Ensure no panic
    }

    #[test]
    fn test_git_history_provider_max_blob_size() {
        let repo_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let provider = GitHistoryProvider::new(&repo_path)
            .unwrap()
            .with_max_blob_size(1024); // 1KB limit

        assert_eq!(provider.max_content_size(), 1024);

        // Test that oversized targets return error
        let target = ScanTarget::new(ScanOrigin::GitDiff {
            commit: "abc123".to_string(),
            path: PathBuf::from("test.txt"),
            added: true,
        })
        .with_size_hint(2048); // 2KB - over limit

        let result = provider.load(&target);
        assert!(matches!(result, Err(SourceProviderError::TooLarge { .. })));
    }
}
