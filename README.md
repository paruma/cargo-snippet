# cargo-snippet

[![CI](https://github.com/paruma/cargo-snippet/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/paruma/cargo-snippet/actions/workflows/ci.yml)

This is a fork of [hatoo/cargo-snippet](https://github.com/hatoo/cargo-snippet), a snippet extractor for competitive programming.

## Installing

You need to install `rustfmt` to run `cargo-snippet`.

```bash
$ rustup component add rustfmt
```

Install this fork from Git. It is not published on crates.io.

```bash
$ cargo install --git https://github.com/paruma/cargo-snippet.git --features="binaries"
```

## Usage

Create a project for snippet.

```
$ cargo new --lib mysnippet
```

Add dependencies to Cargo.toml.

```toml
[dependencies]
cargo-snippet = { git = "https://github.com/paruma/cargo-snippet.git" }
```

This dependency registers the `#[snippet]` attribute so the code compiles. The installed command extracts the snippets.

Then write some snippet codes and tests.

```rust
use cargo_snippet::snippet;

// Annotate snippet name
#[snippet("mymath")]
#[snippet("gcd")]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Also works
#[snippet(name = "mymath")]
// Equivalent to #[snippet("lcm")]
#[snippet]
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[snippet]
// Include snippet
#[snippet(include = "gcd")]
fn gcd_list(list: &[u64]) -> u64 {
    list.iter().fold(list[0], |a, &b| gcd(a, b))
}

// You can set prefix string.
// Note: All codes will be formatted by rustfmt on output
#[snippet(prefix = "use std::io::{self,Read};")]
#[snippet(prefix = "use std::str::FromStr;")]
fn foo() {}

// By default, doc comments associated with items will be output with the snippet.
#[snippet]
/// This is a document!
fn documented() {
    //! Inner document also works.
}

// If you want doc comment to be hidden, append `doc_hidden` keyword.
#[snippet(doc_hidden, prefix = "use std::collections::HashMap;")]
/// This is a doc comment for `bar`.
/// Since `doc_hidden` is specified, it won't be present in the snippet.
fn bar() {
    //! And this is also a doc comment for `bar`, which will be removed.
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(57, 3), 3);
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(3, 19), 57);
}
```

You can test as always:

```
$ cargo test
```

Extract snippet !

```
$ cargo snippet
snippet bar
    use std::collections::HashMap;
    fn bar() {}

snippet documented
    // By default, doc comments associated with items will be output with the snippet.
    /// This is a document!
    fn documented() {
        //! Inner document also works.
    }

snippet foo
    use std::io::{self, Read};
    use std::str::FromStr;
    // You can set prefix string.
    // Note: All codes will be formatted by rustfmt on output
    fn foo() {}

snippet gcd
    // Annotate snippet name
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

snippet gcd_list
    // Annotate snippet name
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    // Include snippet
    fn gcd_list(list: &[u64]) -> u64 {
        list.iter().fold(list[0], |a, &b| gcd(a, b))
    }

snippet lcm
    // Also works
    // Equivalent to #[snippet("lcm")]
    fn lcm(a: u64, b: u64) -> u64 {
        a / gcd(a, b) * b
    }

snippet mymath
    // Annotate snippet name
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }
    // Also works
    // Equivalent to #[snippet("lcm")]
    fn lcm(a: u64, b: u64) -> u64 {
        a / gcd(a, b) * b
    }
```

## Supported output format

* Neosnippet
* VScode
* Ultisnips

You can specify output format via `-t` option.
See `cargo snippet -h`.
