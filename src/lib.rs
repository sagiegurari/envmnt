#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bindings_with_variant_name,
    broken_intra_doc_links,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_in_future,
    drop_bounds,
    ellipsis_inclusive_range_patterns,
    explicit_outlives_requirements,
    exported_private_dependencies,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    inline_no_sanitize,
    invalid_codeblock_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_crate_level_docs,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_autolinks,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    panic_fmt,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_doc_tests,
    private_in_public,
    private_intra_doc_links,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unsupported_naked_functions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
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
    useless_deprecated,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    invalid_html_tags,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

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
//! use envmnt::{ExpandOptions, ExpansionType};
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
//!
//!     envmnt::set("MY_ENV_VAR2", "SOME VALUE2");
//!
//!     let value = envmnt::get_any(&vec!["MY_ENV_VAR1", "MY_ENV_VAR2"], "default");
//!     println!("MY_ENV_VAR1 exists: {}", envmnt::exists("MY_ENV_VAR1"));
//!     println!("MY_ENV_VAR2 exists: {}", envmnt::exists("MY_ENV_VAR2"));
//!     println!("Found value: {}", value);
//!
//!     let mut options = ExpandOptions::new();
//!     options.expansion_type = Some(ExpansionType::Unix);
//!     let mut value = envmnt::expand("Env: MY_ENV value is: ${MY_ENV}", Some(options));
//!     println!("Expanded: {}", &value);
//!     options.expansion_type = Some(ExpansionType::UnixBracketsWithDefaults);
//!     value = envmnt::expand(
//!         "Env: MY_ENV_NOT_FOUND value is: ${MY_ENV_NOT_FOUND:default value}",
//!         Some(options),
//!     );
//!     println!("Expanded: {}", &value);
//! }
//! ```
//!
//! ## Get/Set boolean environment variables and other comparisons
//!
//! ```
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
//!     let mut contains = envmnt::contains("MY_ENV_VAR", "_ENV_");
//!     println!("Value Contained: {}", &contains);
//!     contains = envmnt::contains_ignore_case("MY_ENV_VAR", "_env_");
//!     println!("Value Contained (case insensitive): {}", &contains);
//! }
//! ```
//!
//! ## Get/Set numeric environment variables
//!
//! ```
//! fn main() {
//!     // all numeric data types: u8/i8/u16/i16/u32/i32/u64/i64/u128/i128/f32/f64/isize/usize
//!     // are supported by specific set/get functions.
//!     envmnt::set_u8("U8_TEST_ENV", 50);
//!     let mut value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
//!     println!("u8 value: {}", value_u8);
//!
//!     envmnt::set_isize("ISIZE_TEST_ENV", -50);
//!     let mut value_isize = envmnt::get_isize("ISIZE_TEST_ENV", 5);
//!     println!("isize value: {}", value_isize);
//!
//!     // increment/decrement values
//!     value_isize = envmnt::increment("U8_TEST_ENV");
//!     assert_eq!(value_isize, 51);
//!     value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
//!     assert_eq!(value_u8, 51);
//!     value_isize = envmnt::decrement("U8_TEST_ENV");
//!     assert_eq!(value_isize, 50);
//!     value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
//!     assert_eq!(value_u8, 50);
//! }
//! ```
//!
//! ## Get/Set list environment variables
//!
//! ```
//! fn main() {
//!     envmnt::set_list(
//!         "LIST_TEST_ENV",
//!         &vec!["1".to_string(), "2".to_string(), "3".to_string()],
//!     );
//!
//!     let mut values = envmnt::get_list("LIST_TEST_ENV").unwrap();
//!     println!("List Values: {:?}", values);
//!
//!     let mut same = envmnt::is_equal("LIST_TEST_ENV", "1;2;3");
//!     println!("Same: {}", same);
//!
//!     let mut options = envmnt::ListOptions::new();
//!     options.separator = Some(",".to_string());
//!     envmnt::set_list_with_options(
//!         "LIST_TEST_ENV",
//!         &vec!["1".to_string(), "2".to_string(), "3".to_string()],
//!         &options,
//!     );
//!
//!     values = envmnt::get_list_with_options("LIST_TEST_ENV", &options).unwrap();
//!     println!("List Values: {:?}", values);
//!
//!     same = envmnt::is_equal("LIST_TEST_ENV", "1,2,3");
//!     println!("Same: {}", same);
//! }
//! ```
//!
//! ## Bulk Operations
//!
//! ```
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
//!     envmnt::remove_all(&vec!["ENV_VAR1", "ENV_VAR2"]);
//!
//!     found = envmnt::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);
//!
//!     println!("Any Found: {}", &found);
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

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod bulk;
mod environment;
mod errors;
mod expansion;
mod file;
mod numeric;
pub mod types;
mod util;

use crate::errors::EnvmntError;
use indexmap::IndexMap;
use std::ffi::OsStr;
use std::fmt::Display;
use std::str::FromStr;

/// Get/Set list options
pub type ListOptions = types::ListOptions;

/// Expansion Type - unix/windows style
pub type ExpansionType = types::ExpansionType;

/// Expand options
pub type ExpandOptions = types::ExpandOptions;

/// Returns true environment variable is defined.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
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

/// Removes all provided environment variables.
///
/// # Arguments
///
/// * `keys` - A list of environment variables to remove
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set("MY_ENV_VAR1", "SOME VALUE 1");
///     envmnt::set("MY_ENV_VAR2", "SOME VALUE 2");
///     assert!(envmnt::is_equal("MY_ENV_VAR1", "SOME VALUE 1"));
///     assert!(envmnt::is_equal("MY_ENV_VAR2", "SOME VALUE 2"));
///
///     envmnt::remove_all(&vec!["MY_ENV_VAR1", "MY_ENV_VAR2"]);
///     assert!(!envmnt::exists("MY_ENV_VAR1"));
///     assert!(!envmnt::exists("MY_ENV_VAR2"));
/// }
/// ```
pub fn remove_all<K: AsRef<OsStr>>(keys: &Vec<K>) {
    bulk::remove_all(keys)
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

/// Returns the first environment variable found.
///
/// # Arguments
///
/// * `keys` - The environment variables to search
/// * `default_value` - In case the environment variables are not defined, this value will be returned.
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set("MY_ENV_VAR2", "SOME VALUE2");
///
///     let value = envmnt::get_any(&vec!["MY_ENV_VAR1", "MY_ENV_VAR2"], "default");
///     assert!(!envmnt::exists("MY_ENV_VAR1"));
///     assert!(envmnt::exists("MY_ENV_VAR2"));
///     assert_eq!(value, "SOME VALUE2");
/// }
/// ```
pub fn get_any<K: AsRef<OsStr>>(keys: &Vec<K>, default_value: &str) -> String {
    environment::get_any(keys, default_value)
}

/// Returns false if environment variable value if falsy.<br>
/// Any other value is returned as true.<br>
/// The value is falsy if it is one of the following:
/// * Empty string
/// * "false" (case insensitive)
/// * "no" (case insensitive)
/// * "0"
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `default_value` - In case the environment variable is not defined, this value will be returned.
///
/// # Example
///
/// ```
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
/// <br>
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

/// Sets the environment variable if the provided option contains a value.
/// If no value was provided, the environment variable will be removed.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The optional value to set, none to remove
///
/// # Example
///
/// ```
/// fn main() {
///     let mut output = envmnt::set_or_remove("MY_ENV_VAR", &Some("OPTIONAL VALUE"));
///
///     assert!(output);
///     assert!(envmnt::is_equal(
///         "MY_ENV_VAR",
///         "OPTIONAL VALUE"
///     ));
///
///     output = envmnt::set_or_remove::<&str, &str>("MY_ENV_VAR", &None);
///
///     assert!(!output);
///     assert!(!envmnt::exists("MY_ENV_VAR"));
/// }
/// ```
pub fn set_or_remove<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: &Option<V>) -> bool {
    environment::set_or_remove(key, value)
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

/// Returns true if the provided environment variable is defined and contains the provided value.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The value to check
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME TEST VALUE");
///
///     let contains = envmnt::contains("MY_ENV_VAR", "TEST");
///     assert!(contains);
/// }
/// ```
pub fn contains<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    environment::contains(key, value)
}

/// Returns true if the provided environment variable is defined and contains the provided value regardless of the case.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `value` - The value to check
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set("MY_ENV_VAR", "SOME TEST VALUE");
///
///     let contains = envmnt::contains_ignore_case("MY_ENV_VAR", "test");
///     assert!(contains);
/// }
/// ```
pub fn contains_ignore_case<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    environment::contains_ignore_case(key, value)
}

/// Sets the provided string vector as an environment variable.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `values` - String vector of values
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set_list(
///         "LIST_TEST_ENV",
///         &vec!["1".to_string(), "2".to_string(), "3".to_string()],
///     );
///
///     let values = envmnt::get_list("LIST_TEST_ENV").unwrap();
///     assert_eq!(
///         values,
///         vec!["1".to_string(), "2".to_string(), "3".to_string()]
///     );
/// }
/// ```
pub fn set_list<K: AsRef<OsStr>>(key: K, values: &Vec<String>) {
    environment::set_list(key, values)
}

/// Returns the requested environment variable as a string vector.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set_list(
///         "LIST_TEST_ENV",
///         &vec!["1".to_string(), "2".to_string(), "3".to_string()],
///     );
///
///     let values = envmnt::get_list("LIST_TEST_ENV").unwrap();
///     assert_eq!(
///         values,
///         vec!["1".to_string(), "2".to_string(), "3".to_string()]
///     );
/// }
/// ```
pub fn get_list<K: AsRef<OsStr>>(key: K) -> Option<Vec<String>> {
    environment::get_list(key)
}

/// Sets the provided string vector as an environment variable.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `values` - String vector of values
/// * `options` - See ListOptions
///
/// # Example
///
/// ```
/// fn main() {
///     let mut options = envmnt::ListOptions::new();
///     options.separator = Some(",".to_string());
///     envmnt::set_list_with_options(
///         "LIST_TEST_ENV",
///         &vec!["1".to_string(), "2".to_string(), "3".to_string()],
///         &options,
///     );
///
///     let values = envmnt::get_list_with_options("LIST_TEST_ENV", &options).unwrap();
///     println!("List Values: {:?}", values);
///
///     let same = envmnt::is_equal("LIST_TEST_ENV", "1,2,3");
///     println!("Same: {}", same);
/// }
/// ```
pub fn set_list_with_options<K: AsRef<OsStr>>(key: K, values: &Vec<String>, options: &ListOptions) {
    environment::set_list_with_options(key, values, options)
}

/// Returns the requested environment variable as a string vector.
///
/// # Arguments
///
/// * `key` - The environment variable name
/// * `options` - See ListOptions
///
/// # Example
///
/// ```
/// fn main() {
///     let mut options = envmnt::ListOptions::new();
///     options.separator = Some(",".to_string());
///     envmnt::set_list_with_options(
///         "LIST_TEST_ENV",
///         &vec!["1".to_string(), "2".to_string(), "3".to_string()],
///         &options,
///     );
///
///     let values = envmnt::get_list_with_options("LIST_TEST_ENV", &options).unwrap();
///     println!("List Values: {:?}", values);
///
///     let same = envmnt::is_equal("LIST_TEST_ENV", "1,2,3");
///     println!("Same: {}", same);
/// }
/// ```
pub fn get_list_with_options<K: AsRef<OsStr>>(
    key: K,
    options: &ListOptions,
) -> Option<Vec<String>> {
    environment::get_list_with_options(key, options)
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
/// fn main() {
///     let env = envmnt::parse_file("./src/test/var.env").unwrap();
///
///     println!("Parsed Env: {:?}", &env);
/// }
/// ```
pub fn parse_file(file: &str) -> Result<IndexMap<String, String>, EnvmntError> {
    file::parse_file(file)
}

/// Expands the provided string value by replacing the environment variables defined in it.
/// The syntax of the environment variables is based on the type requested.
///
/// # Arguments
///
/// * `value` - The value to expand
/// * `expansion_type` - The expanstion type (unix/windows/unix prefix/...)
///
/// # Example
///
/// ```
/// use envmnt::{ExpandOptions, ExpansionType};
///
/// fn main() {
///     envmnt::set("MY_ENV", "my expanded value");
///     let mut options = ExpandOptions::new();
///     options.expansion_type = Some(ExpansionType::Unix);
///     let value = envmnt::expand("Env: MY_ENV value is: ${MY_ENV}", Some(options));
///     assert_eq!("Env: MY_ENV value is: my expanded value", &value);
/// }
/// ```
pub fn expand(value: &str, options: Option<ExpandOptions>) -> String {
    environment::expand(&value, options)
}

macro_rules! generate_get_numeric {
    ($fn_name:ident, $type:ty) => {
        /// Returns the environment variable value or a default value
        /// in case the variable is not defined or cannot be parsed.
        ///
        /// # Arguments
        ///
        /// * `key` - The environment variable name
        /// * `default_value` - Returned if the variable does not exist or cannot be parsed
        ///
        pub fn $fn_name<K: AsRef<OsStr>>(key: K, default_value: $type) -> $type {
            numeric::$fn_name(key, default_value)
        }
    };
}

generate_get_numeric!(get_u8, u8);
generate_get_numeric!(get_i8, i8);
generate_get_numeric!(get_i16, i16);
generate_get_numeric!(get_u16, u16);
generate_get_numeric!(get_i32, i32);
generate_get_numeric!(get_u32, u32);
generate_get_numeric!(get_i64, i64);
generate_get_numeric!(get_u64, u64);
generate_get_numeric!(get_i128, i128);
generate_get_numeric!(get_u128, u128);
generate_get_numeric!(get_f32, f32);
generate_get_numeric!(get_f64, f64);
generate_get_numeric!(get_isize, isize);
generate_get_numeric!(get_usize, usize);

macro_rules! generate_set_numeric {
    ($fn_name:ident, $type:ty) => {
        /// Sets the environment variable value.
        ///
        /// # Arguments
        ///
        /// * `key` - The environment variable name
        /// * `value` - The new variable value
        ///
        pub fn $fn_name<K: AsRef<OsStr>>(key: K, value: $type) {
            numeric::$fn_name(key, value)
        }
    };
}

generate_set_numeric!(set_u8, u8);
generate_set_numeric!(set_i8, i8);
generate_set_numeric!(set_i16, i16);
generate_set_numeric!(set_u16, u16);
generate_set_numeric!(set_i32, i32);
generate_set_numeric!(set_u32, u32);
generate_set_numeric!(set_i64, i64);
generate_set_numeric!(set_u64, u64);
generate_set_numeric!(set_i128, i128);
generate_set_numeric!(set_u128, u128);
generate_set_numeric!(set_f32, f32);
generate_set_numeric!(set_f64, f64);
generate_set_numeric!(set_isize, isize);
generate_set_numeric!(set_usize, usize);

/// Increments and returns the new value stored by the given environment variable key.
/// In case the variable does not exist, it will increment to 1.
/// The updated value will be returned.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set_u8("ENV_VAR", 5);
///     let value = envmnt::increment("ENV_VAR");
///     assert_eq!(value, 6);
/// }
/// ```
pub fn increment<K: AsRef<OsStr>>(key: K) -> isize {
    numeric::increment(key)
}

/// Decrements and returns the new value stored by the given environment variable key.
/// In case the variable does not exist, it will decrement to -1.
/// The updated value will be returned.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set_u8("ENV_VAR", 5);
///     let value = envmnt::decrement("ENV_VAR");
///     assert_eq!(value, 4);
/// }
/// ```
pub fn decrement<K: AsRef<OsStr>>(key: K) -> isize {
    numeric::decrement(key)
}

mod generic;


/// Returns the parsed environment variable value.
///
/// # Arguments
///
/// * `key` - The environment variable name
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set("ENV_VAR", "123");
///     let value: i32 = envmnt::get_parse("ENV_VAR").unwrap();
///     assert_eq!(value, 123);
/// }
/// ```
pub fn get_parse<K, T, E>(key: K) -> Result<T, EnvmntError>
    where K: AsRef<OsStr>,
          T: FromStr + FromStr<Err=E>,
          E: Display
{
    generic::_get_parse(key)
}


/// Returns the parsed environment variable value or if is not defined, the default value will be returned.
///
/// # Arguments
///
/// * `key`     - The environment variable name
/// * `default` - The default value to use in case env var is not set
///
/// # Example
///
/// ```
/// fn main() {
///     envmnt::set("ENV_VAR", "123");
///
///     let value: i32 = envmnt::get_parse_or("ENV_VAR", 321).unwrap();
///     assert_eq!(value, 123);
///
///     let value: i32 = envmnt::get_parse_or("ENV_MISSING_VAR", 321).unwrap();
///     assert_eq!(value, 321);
/// }
/// ```
pub fn get_parse_or<K, T, E>(key: K, default: T) -> Result<T, EnvmntError>
    where K: AsRef<OsStr>,
          T: FromStr + FromStr<Err=E>,
          E: Display
{
    generic::_get_parse_or(key, default)
}
