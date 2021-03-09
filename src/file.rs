//! # file
//!
//! File operation.
//!

#[cfg(test)]
#[path = "./file_test.rs"]
mod file_test;

use crate::bulk;
use crate::errors::EnvmntError;
use crate::types::EnvmntResult;
use fsio::error::FsIOError;
use fsio::file::read_text_file;
use indexmap::IndexMap;

pub(crate) fn empty_evaluate_fn(value: String) -> String {
    value
}

pub(crate) fn load_file(file: &str) -> EnvmntResult<()> {
    evaluate_and_load_file(file, empty_evaluate_fn)
}

pub(crate) fn evaluate_and_load_file<F>(file: &str, evaluate: F) -> EnvmntResult<()>
where
    F: Fn(String) -> String,
{
    match parse_file(file) {
        Ok(env) => {
            bulk::evaluate_and_set_all(&env, evaluate);

            Ok(())
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn parse_file(file: &str) -> EnvmntResult<IndexMap<String, String>> {
    match read_text_file(file) {
        Ok(env_content) => {
            let env = parse_env_file_content(&env_content);

            Ok(env)
        }
        Err(error) => Err(create_read_file_error(error)),
    }
}

fn create_read_file_error(error: FsIOError) -> EnvmntError {
    EnvmntError::ReadFile("Unable to read file.", error)
}

pub(crate) fn parse_env_file_content(env_content: &str) -> IndexMap<String, String> {
    let mut env: IndexMap<String, String> = IndexMap::new();

    let lines: Vec<&str> = env_content.split('\n').collect();

    for mut line in lines {
        line = line.trim();

        if line.len() > 0 && !line.starts_with("#") {
            let env_part: Vec<&str> = line.splitn(2, '=').collect();

            if env_part.len() == 2 {
                let key = env_part[0].trim().to_string();
                let mut value = env_part[1].trim().to_string();

                value = value
                    .replace("\\\"", "\"")
                    .replace("\\n", "\n")
                    .replace("\\r", "\r");

                if value.starts_with('"') && value.ends_with('"') {
                    value.remove(0);
                    value.pop();
                }

                env.insert(key, value);
            }
        }
    }

    env
}
