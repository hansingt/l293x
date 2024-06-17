//! # l293x
//! A platform independent, `no_std` driver to interface the
//! [L293 and L293D](https://www.ti.com/lit/ds/symlink/l293.pdf)
//! (Quadruple Half-H Driver) chips.
//!
//! This crate uses [`embedded-hal`](embedded_hal) traits to allow it to be reused in on
//! multiple platforms and boards.
//!
//! ## Basic usage
//! Include the library as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! l293x = { version = "0.1.1" }
//! ```
//!
//! Then you can instantiate a new L293x chip using the corresponding pins
//! from `embedded-hal`.
//!
//! E.g. with an ESP32 Chip using the `esp-hal` crate:
//!
//! ```
//! #[no_std]
//! #[no_main]
//! use esp_hal::{
//!     gpio::{Io, Level, Output},
//!     peripherals::Peripherals,
//!     prelude::*,
//! };
//! use embedded_hal::digital::OutputPin;
//! use l293x::L293x;
//!
//! #[entry]
//! fn main() -> ! {
//!     // Initialize the peripherals
//!     let peripherals = Peripherals::take();
//!     let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
//!
//!     // Create a L293x half bridge motor driver
//!     let mut l293x = L293x::new(
//!         Output::new(io.pins.gpio34, Level::Low),  // Input1
//!         Output::new(io.pins.gpio35, Level::Low),  // Input2
//!         Output::new(io.pins.gpio32, Level::Low),  // Enable12
//!         Output::new(io.pins.gpio33, Level::Low),  // Input3
//!         Output::new(io.pins.gpio25, Level::Low),  // Input4
//!         Output::new(io.pins.gpio26, Level::Low),  // Enable34
//!     ).unwrap();
//!
//!     loop {
//!         for i in 1..=4 {
//!             match i {
//!                 1 => l293x.toggle_output1().unwrap(),
//!                 2 => l293x.toggle_output2().unwrap(),
//!                 3 => l293x.toggle_output3().unwrap(),
//!                 4 => l293x.toggle_output4().unwrap(),
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! ## Usage as Motor driver
//!
//! The L293 and L293D drivers will most likely be used as motor drivers. Thus, it is
//! possible to pass [`embedded_hal::pwm::SetDutyCycle`] pins as inputs as well. This
//! allows to explicitly set the duties of the different outputs, which allows to control
//! the speed of the motor(s):
//!
//! ```
//! use l293x::L293x;
//!
//! // [...] create the PWM pins
//! let mut motors = L293x::new(
//!     m1_forward,
//!     m1_reverse,
//!     m1_enable,
//!     m2_forward,
//!     m2_reverse,
//!     m2_enable,
//! ).unwrap();
//!
//! loop {
//!     // Full speed forward
//!     motors.set_outpu1_duty_cycle_fully_on();
//!     motors.set_outpu3_duty_cycle_fully_on();
//!     delay::delay_millis(500);
//!     motors.set_output1_duty_cycle_fully_off();
//!     motors.set_output3_duty_cycle_fully_off();
//!
//!     // Full speed reverse
//!     motors.set_output2_duty_cycle_fully_on();
//!     motors.set_output4_duty_cycle_fully_on();
//!     delay::delay_millis(500);
//!     motors.set_output2_duty_cycle_fully_off();
//!     motors.set_output4_duty_cycle_fully_off();
//! }
//! ```
//!
//! ## Using only parts of the L293x chip
//!
//! The four Half-H bridges of the L293 Chip can be use independent of each other. Because of this,
//! if you only want to use parts of the chip, you could pass the empty type (`()`) instead of a
//! real pin for the inputs not connected:
//!
//! ```
//! # use l293x::L293x;
//! let mut l293x = L293x::new(input1, (), (), (), enable12, ());
//! ```
//!
//! This causes the type to only implement the functions for the matching outputs.
//!
//! ```compile_fail
//! # use l293x::L293x;
//! # let mut l293x = L293x::new(input1, (), (), (), enable12, ());
//! l293x.set_y1_high()?;  // <-- Ok!
//! l293x.set_y2_high()?;  // <-- Does not compile!
//! ```
//!
//! Because parts of the functionalities of the output relies on the enable pins
//! to be connected, leaving the enable pins not connected does not work. Instead, if your enable
//! pin is always connected to Vcc, the [Vcc](crate::pins::Vcc) struct can be used to express this:
//!
//! ```
//! # use l293x::L293x;
//! use l293x::pins::Vcc;
//!
//! let mut l293x = L293x::new(input1, (), (), (), Vcc(), ());
//! ```
//!
//! For more information, see the struct documentation.
#![no_std]
#![deny(unstable_features, unsafe_code)]
#![cfg_attr(all(coverage_nightly, test), allow(unstable_features))]
#![cfg_attr(all(coverage_nightly, test), feature(coverage_attribute))]

mod l293x;

#[cfg(test)]
mod mock;

// Exports
mod output_state_error;
pub mod pins;

pub use l293x::*;
pub use output_state_error::*;
