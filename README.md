[![Latest Version](https://img.shields.io/crates/v/l293x?logo=Rust)](https://crates.io/crates/l293x)
![Minimum Supported Rust Version](https://img.shields.io/crates/msrv/l293x?logo=Rust)
[![License](https://img.shields.io/crates/l/l293x)](https://crates.io/crates/l293x)
[![Docs](https://img.shields.io/docsrs/l293x?logo=docs.rs)](https://docs.rs/l293x/latest/l293x/)
[![CI](https://github.com/hansingt/l293x/actions/workflows/ci.yml/badge.svg)](https://github.com/hansingt/l293x/actions/workflows/ci.yml)
[![Code Coverage](https://codecov.io/gh/hansingt/l293x/graph/badge.svg?token=udooUR2bx7)](https://codecov.io/gh/hansingt/l293x)

# l293x
A platform independent, `no_std` driver to interface the
[L293 and L293D](https://www.ti.com/lit/ds/symlink/l293.pdf) (Quadruple Half-H Driver)
chips.

This crate uses [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits
to allow it to be reused in on multiple platforms and boards.

## Features

- Drivers for a Half-H Bridge and L293\[D\] chip
- Support for digital and PWM pins
- Support for stateful digital pins

## Minimum Supported Rust Version (MSRV)

This crate is guaranteed to compile on stable Rust 1.60 and up.
It *might* compile with older versions but that may change in any new patch release.

The MSRV may be updated according to the rules defined by
[embedded-hal](https://github.com/rust-embedded/embedded-hal/blob/HEAD/docs/msrv.md).

## License

Licensed under the MIT license
(either [LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
