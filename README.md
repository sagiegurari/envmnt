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
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
This library has many helper functions to access/modify/check environment variables.

<a name="usage"></a>
## Usage
Simply include the library and invoke the various utility functions, for example:

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

    let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");

    let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("New Env Value: {}", &value);
    println!("Previous Env Value: {:?}", &pre_value);

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
    let flag_value = envmnt::is_or("FLAG_VAR", false);
    println!("Bool Flag: {}", &flag_value);

    let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");

    envmnt::set("MY_ENV_VAR", "SOME VALUE");
    let same = envmnt::is_equal("MY_ENV_VAR", "SOME VALUE");
    println!("Value Is Same: {}", &same);
}
```

**Bulk Operations**

```rust
extern crate envmnt;

fn main() {
    let mut found = envmnt::is_any_exists(&vec![
        "ENV_VAR1",
        "ENV_VAR2",
    ]);

    println!("Any Found: {}", &found);

    found = envmnt::is_all_exists(&vec![
        "ENV_VAR1",
        "ENV_VAR2",
    ]);

    println!("All Found: {}", &found);
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

**v0.3.0 (2019-05-10)**

* Added new get_or_panic function #5
* Added new get all args function #6

**v0.2.0 (2019-05-09)**

* Added bulk variables exists operations #3
* Apis support for OS strings #2
* Fix documentation #1

**v0.1.1 (2019-05-08)**

* Added 'is_equal' function

**v0.1.0 (2019-05-08)**

* Initial release

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
