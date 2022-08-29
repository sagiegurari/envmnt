## CHANGELOG

### v0.10.3

* Fix: Expansion bugs (defaults broken and ignored invalid chars) #33 #35

### v0.10.2 (2022-08-13)

* Dependencies upgrade

### v0.10.1 (2022-08-06)

* Dependencies upgrade

### v0.10.0 (2022-04-22)

* \[**backward compatibility break**\] Enhancement: evaluate function accepts both key and value and returns option

### v0.9.1 (2021-09-13)

* Update dependencies

### v0.9.0 (2021-03-09)

* New get_parse and get_parse_or functions #27 (thanks @lweberk)
* New parse_env_file_content function
* Make errors chainable (thanks @lweberk)

### v0.8.4 (2020-07-21)

* Set/Get functions for numeric values #23
* New increment/decrement functions #24

### v0.8.3 (2020-06-18)

* Double quotes value support with backslashes-escaped for env files #22 (thanks @joseluisq)

### v0.8.2 (2020-04-14)

* New expansion type: UnixBracketsWithDefaults which supports expansion with embedded default #19

### v0.8.1 (2020-02-17)

* Use fsio crate for file system apis.

### v0.8.0 (2020-01-18)

* Rust upgrade

### v0.7.5 (2020-01-03)

* New **set_or_remove** function

### v0.7.4 (2019-11-12)

* Imporve **expand** performance.

### v0.7.3 (2019-11-08)

* New **expand** function #15

### v0.7.0 (2019-09-28)

* New **remove_all(&vec)** function #12
* New **contains** and **contains_ignore_case** function #13
* New **get_any** to return first found environment variable #11

### v0.6.0 (2019-06-02)

* New vec<string> list get/set functions #10

### v0.4.0 (2019-05-18)

* Added new **is** function
* Added new **evaluate_and_set_all** function
* Added new **set_optional** function
* Internal changes to EnvmntError

### v0.3.1 (2019-05-17)

* Added new **set_all** function
* Added new **load_file** function #7
* Added new **evaluate_and_load_file** function #7
* Added new **parse_fil**e function #7

### v0.3.0 (2019-05-10)

* Added new **get_or_panic** function #5
* Added new **get all args** function #6

### v0.2.0 (2019-05-09)

* Added bulk variables exists functions #3
* Apis support for OS strings #2
* Fix documentation #1

### v0.1.1 (2019-05-08)

* Added **is_equal** function

### v0.1.0 (2019-05-08)

* Initial release
