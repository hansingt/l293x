//! # Shared pin
//!
//! **NOTE: This module is for internal use only! It is exposed only, because it is used in
//! public interface definitions**
//!
//! This module implements a "shared pin". A pin, which is used by multiple components of the IC.
//! Because of this, a "shared pin" must be cloneable to allow controlling it from multiple
//! components of the IC.
//!
//! In this crate, the [SharedPin] is used by the [L293x](crate::L293x) struct to define the two
//! shared enable pins of the four [HalfH](crate::bridge::HalfH) bridges, which the L293x internally
//! consists of.
//!
//! # Note
//!
//! Please keep in mind, that because of its shared nature, changing the state of a [SharedPin],
//! always has side effects to multiple parts of the IC!
extern crate alloc;

use alloc::rc::Rc;
use core::cell::RefCell;
use embedded_hal::digital::{ErrorType, OutputPin, StatefulOutputPin};

/// A common pin used by multiple parts of the circuit.
///
/// This struct is used in the [L293x](crate::L293x) struct to share the common
/// enable pins with the [Half-H bridges](crate::bridge::HalfH).
///
/// A shared pin is a pin, which always affects all parts using it. Thus, setting the
/// state of this pin will always have side effects to multiple parts of the circuit.
#[derive(Debug)]
#[repr(transparent)]
pub struct SharedPin<P>
where
    P: OutputPin,
{
    pin: Rc<RefCell<P>>,
}

impl<P> SharedPin<P>
where
    P: OutputPin,
{
    pub(crate) fn new(pin: P) -> Self {
        Self {
            pin: Rc::new(RefCell::new(pin)),
        }
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            pin: Rc::clone(&self.pin),
        }
    }
}

impl<P> ErrorType for SharedPin<P>
where
    P: OutputPin,
{
    type Error = P::Error;
}

impl<P> OutputPin for SharedPin<P>
where
    P: OutputPin,
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.pin.borrow_mut().set_low()
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.pin.borrow_mut().set_high()
    }
}

impl<P> StatefulOutputPin for SharedPin<P>
where
    P: StatefulOutputPin,
{
    #[inline]
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        self.pin.borrow_mut().is_set_high()
    }

    #[inline]
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        self.pin.borrow_mut().is_set_low()
    }
}
