//! CFG-Based Vulnerability Slicing Module
//!
//! This module provides control flow graph construction and vulnerability
//! path analysis for detecting exploitable security issues.
//!
//! # Overview
//!
//! The CFG module implements slice-based vulnerability detection:
//! 1. Parse source code with tree-sitter
//! 2. Build control flow graph from AST
//! 3. Detect entry points (HTTP routes, CLI commands, public functions)
//! 4. Detect sink points (SQL, exec, file I/O, network)
//! 5. Backward slice from sinks to find data sources
//! 6. Forward slice from entries to find data sinks
//! 7. Intersect slices to find vulnerability paths
//!
//! # Example
//!
//! ```rust
//! use coax_scanner::cfg::{CFGBuilder, EntryPoint, SinkPoint};
//! use coax_scanner::cfg::slicing::{BackwardSlicer, ForwardSlicer, SliceIntersection};
//!
//! let builder = CFGBuilder::for_language("rust");
//! let cfg = builder.build(source_code)?;
//!
//! let entries = coax_scanner::cfg::entry_points::detect_all(&cfg);
//! let sinks = coax_scanner::cfg::sinks::detect_all(&cfg);
//!
//! let paths = SliceIntersection::find_vulnerability_paths(&cfg, &entries, &sinks);
//! ```

pub mod types;
pub mod builder;
pub mod entry_points;
pub mod sinks;
pub mod backward;
pub mod forward;
pub mod intersection;

pub use types::*;
pub use builder::CFGBuilder;
pub use backward::BackwardSlicer;
pub use forward::ForwardSlicer;
pub use intersection::SliceIntersection;

/// Supported languages for CFG construction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Unknown,
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" => Language::JavaScript,
            "ts" => Language::TypeScript,
            "tsx" => Language::TypeScript,
            _ => Language::Unknown,
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "rust" => Language::Rust,
            "python" => Language::Python,
            "javascript" => Language::JavaScript,
            "typescript" => Language::TypeScript,
            _ => Language::Unknown,
        }
    }
}

/// Error types for CFG operations
#[derive(Debug, thiserror::Error)]
pub enum CFGError {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Language not supported: {0}")]
    LanguageNotSupported(String),

    #[error("CFG construction failed: {0}")]
    Construction(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, CFGError>;
