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

```rust
extern crate envmnt;

fn main() {
    if !envmnt::exists("MY_ENV_VAR") {
        envmnt::set("MY_ENV_VAR", "SOME VALUE");
    }

    let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("Env Value: {}", &value);

    envmnt::set_bool("FLAG_VAR", true);
    let flag_value = envmnt::is_or("FLAG_VAR", false);
    println!("Bool Flag: {}", &flag_value);

    let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");

    let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("New Env Value: {}", &value);
    println!("Previous Env Value: {:?}", &pre_value);
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

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2019-05-08  | v0.1.0  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
