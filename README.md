<h1 align="center">Ferveo</h1>

![ci-badge](https://github.com/nucypher/ferveo/actions/workflows/build.yaml/badge.svg)

# Ferveo

A synchronous Distributed Key Generation protocol for front-running protection on public blockchains.

## About

The distributed key generated by Ferveo can be input into a compatible threshold encryption/decryption or a threshold
signature protocol. Ferveo distributes the shared private key by relative staking weight and relies on an underlying
blockchain for synchonicity.

## Security Warning

This library is under development and has not been reviewed, audited, or prepared for use.

## Documentation

Documentation can be found [here](book/).
It is recommended to use [mdbook](https://rust-lang.github.io/mdBook/) to render the docs. Run `mdbook serve` in
the `book` folder.

A preprint paper describing the construction of Ferveo and the novel cryptosystems used is available at
[IACR](https://eprint.iacr.org/2022/898).

## Build

A Rust toolchain with version `>= 1.53.0` is required. In the future, Ferveo will target the `stable` toolchain.
Installation via [rustup](https://rustup.rs/) is recommended.

Run `cargo build --release` to build.
Please note that performance may be significantly poorer when compiling in `Debug` mode.

## Testing

Run `cargo test --release` to run tests. Please note that performance may be significantly poorer when testing
in `Debug` mode.

## Benchmarks

Run `cargo bench --benches` to run benchmarks. Benchmark report is available in the `target/criterion/report` folder.


