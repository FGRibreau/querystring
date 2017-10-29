`querystring` management for Rust
====================

[![Cargo version](https://img.shields.io/crates/v/querystring.svg)](https://crates.io/crates/querystring)

[![Get help on Codementor](https://cdn.codementor.io/badges/get_help_github.svg)](https://www.codementor.io/francois-guillaume-ribreau?utm_source=github&utm_medium=button&utm_term=francois-guillaume-ribreau&utm_campaign=github)  [![available-for-advisory](https://img.shields.io/badge/available%20for%20advising-yes-ff69b4.svg?)](http://bit.ly/2c7uFJq) ![extra](https://img.shields.io/badge/actively%20maintained-yes-ff69b4.svg?)


The `querystring` crate provides utilities <!-- for parsing and --> formatting URL query strings. It can be accessed using:

```rust
extern crate querystring;

assert_eq!(querystring::stringify(vec![("foo", "bar"), ("baz", "qux")]), "foo=bar&baz=qux&");
```
