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
//! # Example
//!
//! ```
//! extern crate envmnt;
//!
//! fn main() {
//!     if !envmnt::exists("MY_ENV_VAR") {
//!         envmnt::set("MY_ENV_VAR", "SOME VALUE");
//!     }
//!
//!     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
//!     println!("Env Value: {}", &value);
//!
//!     envmnt::set_bool("FLAG_VAR", true);
//!     let flag_value = envmnt::is_or("FLAG_VAR", false);
//!     println!("Bool Flag: {}", &flag_value);
//!
//!     let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");
//!
//!     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
//!     println!("New Env Value: {}", &value);
//!     println!("Previous Env Value: {:?}", &pre_value);
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

mod bulk;
mod environment;
mod util;

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
///
///     envmnt::remove("MY_ENV_VAR");
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
///
///     let value = envmnt::get_remove("MY_ENV_VAR");
///     println!("Env Value: {:?}", &value);
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
///
///     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
///     println!("Env Value: {}", &value);
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
///
///     let value = envmnt::get_or_panic("MY_ENV_VAR");
///     println!("Env Value: {}", &value);
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
///
///     let flag_value = envmnt::is_or("FLAG_VAR", false);
///     println!("Bool Flag: {}", &flag_value);
/// }
/// ```
pub fn is_or<K: AsRef<OsStr>>(key: K, default_value: bool) -> bool {
    environment::is_or(key, default_value)
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
///
///     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
///     println!("Env Value: {}", &value);
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
///
///     let flag_value = envmnt::is_or("FLAG_VAR", false);
///     println!("Bool Flag: {}", &flag_value);
/// }
/// ```
pub fn set_bool<K: AsRef<OsStr>>(key: K, value: bool) {
    environment::set_bool(key, value)
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
///     let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME VALUE");
///
///     let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
///     println!("New Env Value: {}", &value);
///     println!("Previous Env Value: {:?}", &pre_value);
/// }
/// ```
pub fn get_set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) -> Option<String> {
    environment::get_set(key, value)
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
///     println!("Value Is Same: {}", &same);
/// }
/// ```
pub fn is_equal<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    environment::is_equal(key, value)
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
///     let found = envmnt::is_any_exists(&vec![
///         "ENV_VAR1",
///         "ENV_VAR2",
///     ]);
///
///     println!("Any Found: {}", &found);
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
///     let found = envmnt::is_all_exists(&vec![
///         "ENV_VAR1",
///         "ENV_VAR2",
///     ]);
///
///     println!("All Found: {}", &found);
/// }
/// ```
pub fn is_all_exists<K: AsRef<OsStr>>(keys: &Vec<K>) -> bool {
    bulk::is_all_exists(keys)
}
