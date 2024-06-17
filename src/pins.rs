//! # Static Vcc and Ground pin implementations
//!
//! This module implements static pins, which are directly connected either to the Source ([Vcc]) or
//! to the ground ([Gnd]) of the circuit.
//!
//! They can be used to signal that certain pins are always set high or low and cannot be changed.
//! This is required, because the logic of the [L293](crate::L293x) chip partially relies on the
//! knowledge of the state of the output pin.
//!
//! The pins implemented in this module implement the [embedded_hal::digital::OutputPin] and
//! [embedded_hal::digital::StatefulOutputPin] traits to allow requesting the state of the pin.
//! Because the state of the pins cannot be changed, parts of the operations defined in these traits
//! will always fail and return a [OperationNotSupported] error.
//!
//! # Examples
//!
//! To define, that an output of the [L293](crate::L293x) chip is always enabled, the
//! enable pin must be connected to the voltage source of the circuit. This can be expressed, using
//! the [Vcc] struct:
//!
//! ```
//! use l293x::L293x;
//! # use l293x::pins::Vcc;
//!
//! let mut l293x = L293x::new(input1, (), (), (), Vcc(), ());
//! ```
//!
//! On the other hand, it might be required to express, that a certain input pin is always set low
//! and can only be enabled or disabled. For this, the [Gnd] struct can be used:
//!
//! ```
//! use l293x::L293x;
//! # use l293x::pins::Gnd;
//!
//! let mut l293x = L293x::new(Gnd(), (), (), (), enable12, ());
//! ```
use embedded_hal::digital::{Error, ErrorKind, ErrorType, OutputPin, StatefulOutputPin};

/// Error returned by the [Vcc] and [Gnd] structs, when trying to call an operation which is not
/// supported by the pin.
///
/// This can occur, because the [Vcc] and [Gnd] pins implement the
/// [embedded_hal::digital::OutputPin] ad  [embedded_hal::digital::StatefulOutputPin] traits, but
/// they do not allow switching states. In these cases, this error will be returned instead, to
/// signal the caller, that the corresponding operation could not be performed.
#[derive(Debug)]
pub struct OperationNotSupported();

impl Error for OperationNotSupported {
    #[cfg_attr(all(coverage_nightly, test), coverage(off))]
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// A pin which is connected directly to the Vcc source.
///
/// This pin is always high and cannot be set low. Thus, operations trying to change the state
/// of the pin like [`set_low`](Vcc::set_low) or [`toggle`](Vcc::toggle) will return a
/// [OperationNotSupported] error.
///
/// # Examples
///
/// This struct can be used to mark that a given pin is connected to Vcc and thus is always high.
///This is useful, e.g. to define enable pins of the [L293](crate::L293x) chip which are always high
///to always enable the corresponding outputs. This is required, because parts of the chips logic
///rely on the enable pin to be connected (e.g. checking the state of the output).
///
/// ```
/// use l293x::pins::Vcc;
/// use l293x::L293x;
///
/// let mut l293 = L293x::new(input1, (), (), (), Vcc(), ());
/// ```
///
/// The example above defines a L293 chip, which input `A1` is connected to an input pin (`input1`),
/// and which `en12` pin is always set high (Vcc). In this circuit, the L293 chip acts like an
/// operation amplifier, which allows to control a large output voltage with the small voltage of
/// the MCU output.
#[derive(Debug)]
#[repr(transparent)]
pub struct Vcc();

impl ErrorType for Vcc {
    type Error = OperationNotSupported;
}

impl OutputPin for Vcc {
    /// Try to set the Vcc pin low.
    ///
    /// # Errors
    ///
    /// This operation will *always* fail and return a [OperationNotSupported] error.
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Err(OperationNotSupported())
    }

    /// Try to set the Vcc pin high.
    ///
    /// Because the pin is always high, this operation will be a No-Op and always succeed.
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl StatefulOutputPin for Vcc {
    /// Check whether the Vcc pin is set high.
    ///
    /// Because the Vcc pin is always set high, this method will always return `Ok(true)`.
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    /// Check whether the Vcc pin is set low.
    ///
    /// Because the Vcc pin is always set high, this method will always return `Ok(false)`.
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    /// Try to toggle the state of the Vcc pin.
    ///
    /// # Errors
    ///
    /// The Vcc pin is always high and cannot be changed. Because of this, this operation will
    /// *always* fail and return a [OperationNotSupported] error.
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        Err(OperationNotSupported())
    }
}

/// A pin which is connected directly to the ground.
///
/// This pin is always low and cannot be set low. Thus, operations trying to change the state
/// of the pin like [`set_high`](Gnd::set_low) or [`toggle`](Gnd::toggle) will return a
/// [OperationNotSupported] error.
///
/// # Examples
///
/// This struct can be used to mark that a given pin is connected to ground and thus is always low.
/// This is useful, e.g. to define input pins of the [L293](crate::L293x) chip which are always low.
/// This allows to toggle the corresponding output between the high impedance (disabled) and the low
/// state and might be required, depending on the circuit layout.
///
/// ```
/// use l293x::pins::Vcc;
/// use l293x::L293x;
///
/// let mut l293 = L293x::new(Gnd(), (), (), (), enable12, ());
/// ```
#[derive(Debug)]
#[repr(transparent)]
pub struct Gnd();

impl ErrorType for Gnd {
    type Error = OperationNotSupported;
}

impl OutputPin for Gnd {
    /// Try to set the ground pin low.
    ///
    /// Because the ground pin is always low, this operation is a No-Op and directly returns
    /// `Ok(())`.
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Try to set the ground pin high.
    ///
    /// # Errors
    ///
    /// Because the ground pin is always low, this operation will *always* return a
    /// [OperationNotSupported] error.
    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Err(OperationNotSupported())
    }
}

impl StatefulOutputPin for Gnd {
    /// Check whether the ground pin is high.
    ///
    /// Because the ground pin is always low, this method will always return `Ok(false)`.
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(false)
    }

    /// Check whether the ground pin is low.
    ///
    /// Because the ground pin is always low, this method will always return `Ok(true)`.
    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    /// Try to toggle the state of the ground pin.
    ///
    /// # Errors
    ///
    /// The ground pin is always low and cannot be changed. Because of this, this operation will
    /// *always* fail and return a [OperationNotSupported] error.
    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        Err(OperationNotSupported())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use coverage_helper::test;

    #[test]
    fn test_vcc() {
        let mut pin = Vcc();

        assert!(pin.is_set_high().unwrap());
        assert!(!pin.is_set_low().unwrap());
        assert!(matches!(pin.set_high(), Ok(())));
        assert!(matches!(pin.set_low(), Err(OperationNotSupported(..))));
        assert!(matches!(pin.toggle(), Err(OperationNotSupported(..))));
    }

    #[test]
    fn test_ground() {
        let mut pin = Gnd();

        assert!(!pin.is_set_high().unwrap());
        assert!(pin.is_set_low().unwrap());
        assert!(matches!(pin.set_low(), Ok(())));
        assert!(matches!(pin.set_high(), Err(OperationNotSupported(..))));
        assert!(matches!(pin.toggle(), Err(OperationNotSupported(..))));
    }
}
