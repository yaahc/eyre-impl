The shape of errors to come
===============

[![Latest Version](https://img.shields.io/crates/v/eyre-impl.svg)](https://crates.io/crates/eyre-impl)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/eyre-impl)

```toml
[dependencies]
eyre-impl = "0.1"
```

<br>

## Example

The [simple-eyre](https://github.com/yaahc/simple-eyre/blob/master/src/lib.rs)
crate includes a minimalist example of an error type build ontop of eyre-impl
that has a Box<dyn Error> as the error storage type and a Backtrace as the
context.

An error type that builds ontop of eyre-impl should provide the following:
- A `Context` type that contains information about the error (e.g. Backtrace)
  - This `Context` must impl the `Default` trait
  - If you wish to write Result extension traits it should also implement `ErrorContext`
- A `From<std::error::Error>` impl for defining how to convert errors into it
- `Display` and `Debug` impls to format the error, its sources, and the context
  for human consumption

Anything beyond this is just sugar, there is a lot more that can be added, see
[eyre](https://github.com/yaahc/eyre) for a more thorough implementation of an
error type including an Extension trait, a more complex Context, and macros for
error construction.

<br>

## Details

- This library provides the shared machinery needed when building error types
  like `anyhow` or `anomaly`. The key difference in this crate is the clean
  separation of context for errors and the errors themselves. This crate should
  not be used as a direct dependency, the indended usage is to either use one of
  the crates that depend on this crate, or to fork one of those crates and use it
  as a jumping off point for implementing an catch-all contextualizing error type
  that exactly fits your needs.

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>


