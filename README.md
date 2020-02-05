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

```rust
INSERT SIMPLE BOXERROR + BACKTRACE IMPL HERE
```

<br>

## Details

- This library provides the shared machinery needed when building error types
  like `anyhow` or `anomaly`. The key insight in this crate is the clean
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


