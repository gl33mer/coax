//! Unicode Detector Modules
//!
//! This module contains individual detectors for different Unicode attack vectors.

pub mod invisible;
pub mod homoglyph;
pub mod bidi;
pub mod glassworm;
pub mod tags;

pub use invisible::{InvisibleCharDetector, UnicodeDetector as InvisibleDetectorTrait};
pub use homoglyph::{HomoglyphDetector, ConfusableMatch, UnicodeDetector as HomoglyphDetectorTrait};
pub use bidi::{BidiDetector, UnicodeDetector as BidiDetectorTrait};
pub use glassworm::{GlasswormDetector, GlasswormIndicator, UnicodeDetector as GlasswormDetectorTrait};
pub use tags::{UnicodeTagDetector, UnicodeDetector as TagDetectorTrait};

use crate::unicode::config::UnicodeConfig;
use crate::unicode::findings::UnicodeFinding;

/// Unified UnicodeDetector trait for all detectors
pub trait UnicodeDetector: Send + Sync {
    /// Get the name of the detector
    fn name(&self) -> &'static str;
    
    /// Detect attacks in the given content
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    
    /// Check if this detector is enabled in the config
    fn is_enabled(&self, config: &UnicodeConfig) -> bool;
}
