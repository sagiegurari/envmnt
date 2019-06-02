# envmnt

[![crates.io](https://img.shields.io/crates/v/envmnt.svg)](https://crates.io/crates/envmnt) [![Build Status](https://travis-ci.org/sagiegurari/envmnt.svg)](http://travis-ci.org/sagiegurari/envmnt) [![Build status](https://ci.appveyor.com/api/projects/status/yrb4y9cbaf6wtlk7?svg=true)](https://ci.appveyor.com/project/sagiegurari/envmnt) [![codecov](https://codecov.io/gh/sagiegurari/envmnt/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/envmnt)<br>
[![license](https://img.shields.io/crates/l/envmnt.svg)](https://github.com/sagiegurari/envmnt/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/envmnt.svg)](https://libraries.io/cargo/envmnt) [![Documentation](https://docs.rs/envmnt/badge.svg)](https://docs.rs/crate/envmnt/) [![downloads](https://img.shields.io/crates/d/envmnt.svg)](https://crates.io/crates/envmnt)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Environment variables utility functions.

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/envmnt/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](https://github.com/sagiegurari/envmnt/blob/master/CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library has many helper functions to access/modify/check environment variables.

<a name="usage"></a>
## Usage
Simply include the library and invoke the various utility functions.

**Get/Set/Remove environment variables**

```rust
extern crate envmnt;

fn main() {
    if !envmnt::exists("MY_ENV_VAR") {
        envmnt::set("MY_ENV_VAR", "SOME VALUE");
    }

    let mut value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("Env Value: {}", &value);

    value = envmnt::get_or_panic("MY_ENV_VAR");
    println!("Env Value: {}", &value);

    let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");

    let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("New Env Value: {}", &value);
    println!("Previous Env Value: {:?}", &pre_value);

    let var_was_set = envmnt::set_optional("MY_ENV_VAR", &Some("OPTIONAL VALUE"));
    println!("Env Was Modified: {}", var_was_set);

    let all_vars = envmnt::vars(); // returned as Vec<(String, String)>

    for (key, value) in all_vars {
        println!("{}: {}", key, value);
    }
}
```

**Get/Set boolean environment variables and other comparisons**

```rust
extern crate envmnt;

fn main() {
    envmnt::set_bool("FLAG_VAR", true);
    let mut flag_value = envmnt::is_or("FLAG_VAR", false);
    println!("Bool Flag: {}", &flag_value);

    flag_value = envmnt::is("FLAG_VAR");
    assert!(flag_value);

    envmnt::set_bool("FLAG_VAR", true);
    assert!(envmnt::is_equal("FLAG_VAR", "true"));

    envmnt::set("MY_ENV_VAR", "SOME VALUE");
    let same = envmnt::is_equal("MY_ENV_VAR", "SOME VALUE");
    println!("Value Is Same: {}", &same);
}
```

**Get/Set list environment variables**

```rust
extern crate envmnt;

fn main() {
    envmnt::set_list(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let mut values = envmnt::get_list("LIST_TEST_ENV").unwrap();
    println!("List Values: {:?}", values);

    let mut same = envmnt::is_equal("LIST_TEST_ENV", "1;2;3");
    println!("Same: {}", same);

    let mut options = envmnt::ListOptions::new();
    options.separator = Some(",".to_string());
    envmnt::set_list_with_options(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    values = envmnt::get_list_with_options("LIST_TEST_ENV", &options).unwrap();
    println!("List Values: {:?}", values);

    same = envmnt::is_equal("LIST_TEST_ENV", "1,2,3");
    println!("Same: {}", same);
}
```

**Bulk Operations**

```rust
extern crate envmnt;
extern crate indexmap;

use indexmap::IndexMap;

fn main() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());

    envmnt::set_all(&env);

    let value = envmnt::get_or_panic("ENV_VAR1");
    println!("Value Is: {}", &value);

    let mut found = envmnt::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("Any Found: {}", &found);

    found = envmnt::is_all_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("All Found: {}", &found);

    env = IndexMap::new();
    env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());

    let eval_env = |value: String| {
        let mut buffer = String::from("VALUE-");
        buffer.push_str(&value);
        buffer
    };

    envmnt::evaluate_and_set_all(&env, eval_env);

    let value = envmnt::get_or_panic("ENV_VAR1");
    println!("Value Is: {}", &value);
}
```

**File Operations**

```rust
extern crate envmnt;

fn main() {
    let mut output = envmnt::load_file("./src/test/var.env");
    assert!(output.is_ok());

    let eval_env = |value: String| {
        let mut buffer = String::from("PREFIX-");
        buffer.push_str(&value);
        buffer
    };

    output = envmnt::evaluate_and_load_file("./src/test/var.env", eval_env);
    assert!(output.is_ok());
}
```

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
envmnt = "*"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/envmnt/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/envmnt/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
