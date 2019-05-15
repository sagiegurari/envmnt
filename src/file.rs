//! # file
//!
//! File operation.
//!

#[cfg(test)]
#[path = "./file_test.rs"]
mod file_test;

use crate::environment;
use crate::types::{EnvmntError, ErrorInfo};
use indexmap::IndexMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub(crate) fn empty_evaluate_fn(value: String) -> String {
    value
}

pub(crate) fn load_file(file: &str) -> Result<(), EnvmntError> {
    evaluate_and_load_file(file, empty_evaluate_fn)
}

pub(crate) fn evaluate_and_load_file<F>(file: &str, evaluate: F) -> Result<(), EnvmntError>
where
    F: Fn(String) -> String,
{
    match parse_file(file) {
        Ok(env) => {
            for (key, value) in env.iter() {
                let evaluated_value = evaluate(value.to_string());

                environment::set(key.to_string(), evaluated_value);
            }

            Ok(())
        }
        Err(error) => Err(error),
    }
}

pub(crate) fn parse_file(file: &str) -> Result<IndexMap<String, String>, EnvmntError> {
    let file_path = Path::new(&file).to_path_buf();

    if file_path.exists() {
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut env_content = String::new();
                file.read_to_string(&mut env_content).unwrap();

                let env = parse_env_content(&env_content);

                Ok(env)
            }
            Err(_) => Err(create_file_open_error()),
        }
    } else {
        Err(EnvmntError {
            info: ErrorInfo::FileNotFound("File Not Found."),
        })
    }
}

fn create_file_open_error() -> EnvmntError {
    EnvmntError {
        info: ErrorInfo::FileOpen("Unable To Open File."),
    }
}

fn parse_env_content(env_content: &str) -> IndexMap<String, String> {
    let mut env: IndexMap<String, String> = IndexMap::new();

    let lines: Vec<&str> = env_content.split('\n').collect();

    for mut line in lines {
        line = line.trim();

        if line.len() > 0 && !line.starts_with("#") {
            let env_part: Vec<&str> = line.splitn(2, '=').collect();

            if env_part.len() == 2 {
                let key = env_part[0].trim().to_string();
                let value = env_part[1].trim().to_string();

                env.insert(key, value);
            }
        }
    }

    env
}
