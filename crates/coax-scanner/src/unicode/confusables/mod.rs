//! Confusables Module
//!
//! This module provides confusable character detection for homoglyph attacks.

pub mod data;

pub use data::{
    ConfusableEntry,
    CONFUSABLES_DB,
    REVERSE_CONFUSABLES,
    ALL_CONFUSABLES,
    get_confusables,
    get_base_char,
    is_confusable,
    get_confusable_script,
    get_similarity,
};
