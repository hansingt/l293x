[![CI](https://github.com/hansingt/l293x/actions/workflows/ci.yml/badge.svg)](https://github.com/hansingt/l293x/actions/workflows/ci.yml)
[![Code Coverage](https://codecov.io/gh/hansingt/l293x/graph/badge.svg?token=udooUR2bx7)](https://codecov.io/gh/hansingt/l293x)
[![Latest Version](https://img.shields.io/crates/v/l293x)](https://crates.io/crates/l293x)
[![License](https://img.shields.io/crates/l/l293x)](https://crates.io/crates/l293x)

# l293x
A platform independent, `no_std` driver to interface the
[L293 and L293D](https://www.ti.com/lit/ds/symlink/l293.pdf) (Quadruple Half-H Driver)
chips.

This crate uses [`embedded-hal`](https://github.com/rust-embedded/embedded-hal) traits
to allow it to be reused in on multiple platforms and boards.

## Features

- Drivers for a Half-H-Bridge, H-Bridge and L293x chip
- Support for digital and PWM pins
- Support for stateful digital pins
- Splitting the L293x or H-Bridge drivers to usage as inputs to other drivers

## License

Licensed under the MIT license
(either [LICENSE](LICENSE) or http://opensource.org/licenses/MIT).
