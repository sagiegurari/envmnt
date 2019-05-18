#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    duplicate_macro_exports,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incoherent_fundamental_impls,
    invalid_type_param_default,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    nested_impl_trait,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    question_mark_macro_sep,
    safe_extern_statics,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]
#![cfg_attr(feature = "clippy", feature(plugin))]

//! # envmnt
//!
//! Environment variables utility functions.
//!
//! This library has many helper functions to access/modify/check environment variables.
//!
//! # Examples
//!
//! ## Get/Set/Remove environment variables
//!
//! ```
//! extern crate envmnt;
//!
//! fn main() {
//!     if !envmnt::exists("MY_ENV_VAR") {
//!         envmnt::set("MY_ENV_VAR", "SOME VALUE");
//!     }
//!
//!     let mut value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
//!     println!("Env Value: {}", &value);
//!
//!     value = envmnt::get_or_panic("MY_ENV_VAR");
//!     println!("Env Value: {}", &value);
//!
//!     let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");
//!
//!     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
//!     println!("New Env Value: {}", &value);
//!     println!("Previous Env Value: {:?}", &pre_value);
//!
//!     let var_was_set = envmnt::set_optional("MY_ENV_VAR", &Some("OPTIONAL VALUE"));
//!     println!("Env Was Modified: {}", var_was_set);
//!
//!     let all_vars = envmnt::vars(); // returned as Vec<(String, String)>
//!
//!     for (key, value) in all_vars {
//!         println!("{}: {}", key, value);
//!     }
//! }
//! ```
//!
//! ## Get/Set boolean environment variables and other comparisons
//!
//! ```
//! extern crate envmnt;
//!
//! fn main() {
//!     envmnt::set_bool("FLAG_VAR", true);
//!     let mut flag_value = envmnt::is_or("FLAG_VAR", false);
//!     println!("Bool Flag: {}", &flag_value);
//!
//!     flag_value = envmnt::is("FLAG_VAR");
//!     assert!(flag_value);
//!
//!     envmnt::set_bool("FLAG_VAR", true);
//!     assert!(envmnt::is_equal("FLAG_VAR", "true"));
//!
//!     envmnt::set("MY_ENV_VAR", "SOME VALUE");
//!     let same = envmnt::is_equal("MY_ENV_VAR", "SOME VALUE");
//!     println!("Value Is Same: {}", &same);
//! }
//! ```
//!
//! ## Bulk Operations
//!
//! ```
//! extern crate envmnt;
//! extern crate indexmap;
//!
//! use indexmap::IndexMap;
//!
//! fn main() {
//!     let mut env: IndexMap<String, String> = IndexMap::new();
//!     env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
//!     env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());
//!
//!     envmnt::set_all(&env);
//!
//!     let value = envmnt::get_or_panic("ENV_VAR1");
//!     println!("Value Is: {}", &value);
//!
//!     let mut found = envmnt::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);
//!
//!     println!("Any Found: {}", &found);
//!
//!     found = envmnt::is_all_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);
//!
//!     println!("All Found: {}", &found);
//!
//!     env = IndexMap::new();
//!     env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
//!     env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());
//!
//!     let eval_env = |value: String| {
//!         let mut buffer = String::from("VALUE-");
//!         buffer.push_str(&value);
//!         buffer
//!     };
//!
//!     envmnt::evaluate_and_set_all(&env, eval_env);
//!
//!     let value = envmnt::get_or_panic("ENV_VAR1");
//!     println!("Value Is: {}", &value);
//! }
//! ```
//!
//! ## File Operations
//!
//! ```
//! extern crate envmnt;
//!
//! fn main() {
//!     let mut output = envmnt::load_file("./src/test/var.env");
//!     assert!(output.is_ok());
//!
//!     let eval_env = |value: String| {
//!         let mut buffer = String::from("PREFIX-");
//!         buffer.push_str(&value);
//!         buffer
//!     };
//!
//!     output = envmnt::evaluate_and_load_file("./src/test/var.env", eval_env);
//!     assert!(output.is_ok());
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! envmnt = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/envmnt/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/envmnt/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

extern crate indexmap;

mod bulk;
mod environment;
mod errors;
mod file;
mod util;

use crate::errors::EnvmntError;
use indexmap::IndexMap;
use std::ffi::OsStr;

/// Returns true environment variable is defined.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     if !envmnt::exists("MY_ENV_VAR") {
///         envmnt::set("MY_ENV_VAR", "SOME VALUE");
///         assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///     }
/// }
/// ```
pub fn exists<K: AsRef<OsStr>>(key: K) -> bool {
    environment::exists(key)
}

/// Removes the provided environment variable.
///
/// # Arguments
///
/// * `key` - The environment variable to remove
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///     assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///
///     envmnt::remove("MY_ENV_VAR");
///     assert!(!envmnt::exists("MY_ENV_VAR"));
/// }
/// ```
pub fn remove<K: AsRef<OsStr>>(key: K) {
    environment::remove(key)
}

/// Removes the provided environment variable and returns its previous value (if any).
///
/// # Arguments
///
/// * `key` - The environment variable to remove
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///     assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///
///     let value = envmnt::get_remove("MY_ENV_VAR");
///     assert!(!envmnt::exists("MY_ENV_VAR"));
///     assert_eq!(value.unwrap(), "SOME VALUE");
/// }
/// ```
pub fn get_remove<K: AsRef<OsStr>>(key: K) -> Option<String> {
    environment::get_remove(key)
}

/// Returns the environment variable value or if is not defined, the default value will be returned.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `default_value` - In case the environment variable is not defined, this value will be returned.
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///     assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///
///     let mut value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
///     assert_eq!(value, "SOME VALUE");
///
///     value = envmnt::get_or("ANOTHER_ENV_VAR", "DEFAULT_VALUE");
///     assert_eq!(value, "DEFAULT_VALUE");
/// }
/// ```
pub fn get_or<K: AsRef<OsStr>>(key: K, default_value: &str) -> String {
    environment::get_or(key, default_value)
}

/// Returns the environment variable value.
/// If the variable is not defined, this function will panic.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///     assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///
///     let value = envmnt::get_or_panic("MY_ENV_VAR");
///     assert_eq!(value, "SOME VALUE");
/// }
/// ```
pub fn get_or_panic<K: AsRef<OsStr>>(key: K) -> String {
    environment::get_or_panic(key)
}

/// Returns false if environment variable value if falsy.
/// The value is falsy if it is one of the following:
/// * Empty string
/// * "false" (case insensitive)
/// * "no" (case insensitive)
/// * "0"
/// Any other value is returned as true.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `default_value` - In case the environment variable is not defined, this value will be returned.
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set_bool("FLAG_VAR", true);
///     assert!(envmnt::is_equal("FLAG_VAR", "true"));
///
///     let flag_value = envmnt::is_or("FLAG_VAR", false);
///     assert!(flag_value);
/// }
/// ```
pub fn is_or<K: AsRef<OsStr>>(key: K, default_value: bool) -> bool {
    environment::is_or(key, default_value)
}

/// Returns false if environment variable value if falsy.
/// The value is falsy if it is one of the following:
/// * Empty string
/// * "false" (case insensitive)
/// * "no" (case insensitive)
/// * "0"
/// Any other value is returned as true.
/// This is same as calling is_or("varname", false)
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set_bool("FLAG_VAR", true);
///     assert!(envmnt::is_equal("FLAG_VAR", "true"));
///
///     let flag_value = envmnt::is("FLAG_VAR");
///     assert!(flag_value);
/// }
/// ```
pub fn is<K: AsRef<OsStr>>(key: K) -> bool {
    environment::is(key)
}

/// Sets the environment variable value.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The environment variable value
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///     assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///
///     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
///     assert_eq!(value, "SOME VALUE");
/// }
/// ```
pub fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    environment::set(key, value)
}

/// Sets the environment variable with a true/false value as string.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - true/false boolean value which will be converted to string
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set_bool("FLAG_VAR", true);
///     assert!(envmnt::is_equal("FLAG_VAR", "true"));
///
///     let flag_value = envmnt::is_or("FLAG_VAR", false);
///     assert!(flag_value);
/// }
/// ```
pub fn set_bool<K: AsRef<OsStr>>(key: K, value: bool) {
    environment::set_bool(key, value)
}

/// Sets the environment variable if the provided option contains a value.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The optional value to set
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     let output = envmnt::set_optional("MY_ENV_VAR", &Some("OPTIONAL VALUE"));
///
///     assert!(output);
///     assert!(envmnt::is_equal(
///         "MY_ENV_VAR",
///         "OPTIONAL VALUE"
///     ));
/// }
/// ```
pub fn set_optional<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: &Option<V>) -> bool {
    environment::set_optional(key, value)
}

/// Sets the environment variable value and returns the previous value.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The environment variable value
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///     assert!(envmnt::is_equal("MY_ENV_VAR", "SOME VALUE"));
///
///     let pre_value = envmnt::get_set("MY_ENV_VAR", "NEW VALUE");
///
///     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
///     assert_eq!(value, "NEW VALUE");
///     assert_eq!(pre_value.unwrap(), "SOME VALUE");
/// }
/// ```
pub fn get_set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) -> Option<String> {
    environment::get_set(key, value)
}

/// Returns all environment variables as a vector.
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     let all_vars = envmnt::vars();
///
///     for (key, value) in all_vars {
///         println!("{}: {}", key, value);
///     }
/// }
/// ```
pub fn vars() -> Vec<(String, String)> {
    environment::vars()
}

/// Returns true if the provided environment variable is defined and equals the provided value.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The value to check
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME VALUE");
///
///     let same = envmnt::is_equal("MY_ENV_VAR", "SOME VALUE");
///     assert!(same);
/// }
/// ```
pub fn is_equal<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    environment::is_equal(key, value)
}

/// Sets all the provided env key/value pairs.
///
/// # Arguments
///
/// * `env` - The environment variables to set
///
/// # Example
///
/// ```
/// extern crate envmnt;
/// extern crate indexmap;
///
/// use indexmap::IndexMap;
///
/// fn main() {
///     let mut env: IndexMap<String, String> = IndexMap::new();
///     env.insert("MY_ENV_VAR".to_string(), "MY VALUE".to_string());
///     env.insert("MY_ENV_VAR2".to_string(), "MY VALUE2".to_string());
///
///     envmnt::set_all(&env);
///
///     let mut value = envmnt::get_or_panic("MY_ENV_VAR");
///     assert_eq!(value, "MY VALUE");
///     value = envmnt::get_or_panic("MY_ENV_VAR2");
///     assert_eq!(value, "MY VALUE2");
/// }
/// ```
pub fn set_all(env: &IndexMap<String, String>) {
    bulk::set_all(&env)
}

/// Sets all the provided env key/value pairs.
///
/// # Arguments
///
/// * `env` - The environment variables to set
/// * `evaluate` - Evalute function which will modify the read value before it is loaded into the environment
///
/// # Example
///
/// ```
/// extern crate envmnt;
/// extern crate indexmap;
///
/// use indexmap::IndexMap;
///
/// fn main() {
///     let mut env: IndexMap<String, String> = IndexMap::new();
///     env.insert("MY_ENV_VAR".to_string(), "MY VALUE".to_string());
///     env.insert("MY_ENV_VAR2".to_string(), "MY VALUE2".to_string());
///
///     let eval_env = |value: String| {
///         let mut buffer = String::from("VALUE-");
///         buffer.push_str(&value);
///         buffer
///     };
///
///     envmnt::evaluate_and_set_all(&env, eval_env);
///
///     let mut value = envmnt::get_or_panic("MY_ENV_VAR");
///     assert_eq!(value, "VALUE-MY VALUE");
///     value = envmnt::get_or_panic("MY_ENV_VAR2");
///     assert_eq!(value, "VALUE-MY VALUE2");
/// }
/// ```
pub fn evaluate_and_set_all<F>(env: &IndexMap<String, String>, evaluate: F)
where
    F: Fn(String) -> String,
{
    bulk::evaluate_and_set_all(&env, evaluate)
}

/// Returns true if any of environment variables is defined.
///
/// # Arguments
///
/// * `keys` - The environment variable names
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("ENV_VAR1", "SOME VALUE");
///     envmnt::remove("ENV_VAR2");
///
///     let found = envmnt::is_any_exists(&vec![
///         "ENV_VAR1",
///         "ENV_VAR2",
///     ]);
///
///     assert!(found);
/// }
/// ```
pub fn is_any_exists<K: AsRef<OsStr>>(keys: &Vec<K>) -> bool {
    bulk::is_any_exists(keys)
}

/// Returns true if all of environment variables are defined.
///
/// # Arguments
///
/// * `keys` - The environment variable names
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     envmnt::set("ENV_VAR1", "SOME VALUE");
///     envmnt::set("ENV_VAR2", "SOME VALUE");
///
///     let mut found = envmnt::is_all_exists(&vec![
///         "ENV_VAR1",
///         "ENV_VAR2",
///     ]);
///
///     assert!(found);
///
///     envmnt::remove("ENV_VAR2");
///
///     found = envmnt::is_all_exists(&vec![
///         "ENV_VAR1",
///         "ENV_VAR2",
///     ]);
///
///     assert!(!found);
/// }
/// ```
pub fn is_all_exists<K: AsRef<OsStr>>(keys: &Vec<K>) -> bool {
    bulk::is_all_exists(keys)
}

/// Parses the provided env file and loads all environment variables.
///
/// # Arguments
///
/// * `file` - The file path to load and parse
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     let output = envmnt::load_file("./src/test/var.env");
///
///     assert!(output.is_ok());
/// }
/// ```
pub fn load_file(file: &str) -> Result<(), EnvmntError> {
    file::load_file(file)
}

/// Parses the provided env file and loads all environment variables.
///
/// # Arguments
///
/// * `file` - The file path to load and parse
/// * `evaluate` - Evalute function which will modify the read value before it is loaded into the environment
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     let eval_env = |value: String| {
///         let mut buffer = String::from("PREFIX-");
///         buffer.push_str(&value);
///         buffer
///     };
///
///     let output = envmnt::evaluate_and_load_file("./src/test/var.env", eval_env);
///
///     assert!(output.is_ok());
/// }
/// ```
pub fn evaluate_and_load_file<F>(file: &str, evaluate: F) -> Result<(), EnvmntError>
where
    F: Fn(String) -> String,
{
    file::evaluate_and_load_file(file, evaluate)
}

/// Parses the provided env file and returns its content as a map of key/value.
///
/// # Arguments
///
/// * `file` - The file path to load and parse
///
/// # Example
///
/// ```
/// extern crate envmnt;
///
/// fn main() {
///     let env = envmnt::parse_file("./src/test/var.env").unwrap();
///
///     println!("Parsed Env: {:?}", &env);
/// }
/// ```
pub fn parse_file(file: &str) -> Result<IndexMap<String, String>, EnvmntError> {
    file::parse_file(file)
}
