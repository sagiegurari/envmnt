//! # types
//!
//! Public library types.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use crate::errors::EnvmntError;

/// Envmt Library Result
pub type EnvmntResult<T> = Result<T, EnvmntError>;

/// Get/Set list options
#[derive(Debug, Clone, Default)]
pub struct ListOptions {
    /// The separator used to merge/split the values
    pub separator: Option<String>,
    /// if true, empty list will not be set and empty string will be considered as a list with a single empty value
    pub ignore_empty: bool,
}

impl ListOptions {
    /// Creates and returns a new instance.
    pub fn new() -> ListOptions {
        Default::default()
    }
}

/// Expansion Type - unix/windows style
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExpansionType {
    /// Unix prefix environment style, for example: $MY_ENV
    UnixPrefix,
    /// Unix brackets environment style, for example: ${MY_ENV}
    UnixBrackets,
    /// All unix supported styles
    Unix,
    /// Windows environment style, for example: %MY_ENV%
    Windows,
    /// Current OS supported styles (Unix/Windows)
    OS,
    /// All supported styles for all platforms (not including custom types such as UnixBracketsWithDefaults
    All,
    /// Unix brackets like format with default value support ${key:default}
    UnixBracketsWithDefaults,
}

/// Expand options
#[derive(Debug, Copy, Clone, Default)]
pub struct ExpandOptions {
    /// The expansion type (unix/windows/...)
    pub expansion_type: Option<ExpansionType>,
    /// If true (default), empty variables will be replaced with empty text, false to keep original variable untouched.
    /// This is ignored in case of expansion type: UnixBracketsWithDefaults and a default is provided.
    pub default_to_empty: bool,
}

impl ExpandOptions {
    /// Creates and returns a new instance.
    pub fn new() -> ExpandOptions {
        ExpandOptions {
            expansion_type: None,
            default_to_empty: true,
        }
    }

    /// Clones and modifies the expansion type
    pub fn clone_with_expansion_type(
        self: &ExpandOptions,
        expansion_type: ExpansionType,
    ) -> ExpandOptions {
        let mut options = self.clone();

        options.expansion_type = Some(expansion_type);

        options
    }
}
