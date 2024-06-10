//! # Half-H bridge driver
//!
//! This module implements a Half-H bridge driver. It allows controlling a single
//! Half-H bridge as used in the [L293x](crate::L293x) chip.
//!
//! A Half-H bridge is a [DC-to-DC converter](https://en.wikipedia.org/wiki/DC-to-DC_converter),
//! which maps the level of the input to a given circuit voltage. The enable pin allows to enable
//! or disable the converter thus, resulting in three possible output states:
//!
//! | input | enable | output |
//! |:-----:|:------:|:------:|
//! | `L`   | `H`    | `L`    |
//! | `H`   | `H`    | `H`    |
//! | `X`   | `L`    | `Z`    |
//!
//! Where `H` = High, `L` = Low, `X` = Doesn't matter, and `Z` = high impedance
//! (disabled).
//!
//! If the output is disabled (enable = low), the input is not forwarded to the output
//! and the output remains in a high impedance state. Because of this, the level output
//! depends on the components connected to it in this case.
//!
//! # Examples
//!
//! To create a new Half-H bridge, simply pass the input and enable pins to it:
//!
//! ```
//! use l293x::HalfH;
//!
//! let h = HalfH::new(input, enable);
//! ```
//!
//! The enable pin *must* implement the [embedded_hal::digital::OutputPin] trait.
//! The [`enable()`](HalfH::enable) and [`disable()`](HalfH::disable) methods can then
//! be used to control the output of the [HalfH]:
//!
//! If the enable pin implements the [embedded_hal::digital::StatefulOutputPin] trait
//! as well, the [`is_enabled()`](HalfH::is_enabled) and
//! [`is_disabled()`](HalfH::is_disabled) methods can be used, to check the state of the
//! output as well:
//!
//! ```
//! h.enable().unwrap()
//! assert!(h.is_enabled().unwrap());
//!
//! h.disable().unwrap()
//! assert!(h.is_disabled().unwrap());
//! ```
//!
//! Depending on the traits implement by the input pin, a [HalfH] implements the
//! corresponding [embedded_hal] traits as well.
//!
//! ## Stateless output pin
//!
//! If the input implements [embedded_hal::digital::OutputPin], the [HalfH] can be
//! used to set the state of the output (after enabling it):
//!
//! ```
//! h.enable().unwrap();
//! h.set_high();
//! ```
//!
//! ## Stateful output pin
//! If both, the input and enable pins, implement the
//! [embedded_hal::digital::StatefulOutputPin] trait, the state of the output can be
//! requested as well:
//!
//! ```
//! h.enable().unwrap();
//! h.set_high().unwrap();
//! assert!(h.is_set_high().unwrap());
//! ```
//!
//! Please note, that if the output is not enabled, it is considered neither high, nor
//! low. Thus, both methods (
//! [`is_set_high`](HalfH::is_set_high), and [`is_set_low`](HalfH::is_set_low)) will
//! return `false` in this case:
//!
//! ```
//! h.disable().unwrap();
//! assert!(!h.is_set_high().unwrap());
//! assert!(!h.is_set_low().unwrap());
//! ```
//!
//! ## Pulse-With-Modulation (PWM) pin
//!
//! If the input implement the [embedded_hal::pwm::SetDutyCycle] trait, the [HalfH]
//! can be used to set the duty cycle of the output:
//!
//! ```
//! h.set_duty_cycle(0).unwrap();
//! h.set_duty_cycle(h.max_duty_cycle()).unwrap();
//! ```
use embedded_hal::digital::{Error, ErrorKind, ErrorType, OutputPin, StatefulOutputPin};
use embedded_hal::pwm::SetDutyCycle;

/// A Half-H bridge driver
///
/// This driver allows controlling a single Half-H bridge as used in the
/// [L293x](crate::L293x) chip.
///
/// For more information, please see the [module documentation](crate::bridge).
#[derive(Debug)]
pub struct HalfH<I, E> {
    pub(crate) input: I,
    pub(crate) enable: E,
}

/// Error returned by the [HalfH] implementation of the [OutputPin] traits.
///
/// This enumeration combines the possible errors returned by the input pin and the enable pin.
///
/// Depending on the source of the error, either an [InputPinError](OutputStateError::InputPinError)
/// or a [EnablePinError](OutputStateError::EnablePinError) will be returned by the functions
/// implemented in the [OutputPin] traits.
///
/// # Examples
///
/// ```
/// use embedded_hal::digital::OutputPin;
/// use l293x::{HalfH, OutputStateError};
///
/// let mut bridge = HalfH::new(input, enable);
///
/// bridge.set_high().unwrap_or(|error| {
///     match error {
///         OutputStateError::InputPinError(e) => println!("Error setting the input pin high: {e}"),
///         OutputStateError::EnablePinError(e) => println!("Error in enable pin: {e}"),
///     }
/// });
/// ```
#[derive(Debug)]
pub enum OutputStateError<I, E> {
    /// An error occurred while setting the state of the input pin. The contained error
    //     /// may contain additional information.
    InputPinError(I),
    /// An error occurred while setting the state of the enable pin. The contained error
    /// may contain additional information.
    EnablePinError(E),
}

impl<I, E> Error for OutputStateError<I, E>
where
    I: Error,
    E: Error,
{
    fn kind(&self) -> ErrorKind {
        match self {
            OutputStateError::InputPinError(e) => e.kind(),
            OutputStateError::EnablePinError(e) => e.kind(),
        }
    }
}

impl<I, E> PartialEq for OutputStateError<I, E>
where
    I: PartialEq,
    E: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match self {
            OutputStateError::EnablePinError(e1) => match other {
                OutputStateError::EnablePinError(e2) => e1 == e2,
                _ => false,
            },
            OutputStateError::InputPinError(e1) => match other {
                OutputStateError::InputPinError(e2) => e1 == e2,
                _ => false,
            },
        }
    }
}

impl<I, E> Eq for OutputStateError<I, E>
where
    I: Eq,
    E: Eq,
{
}

impl<I, E> HalfH<I, E>
where
    E: OutputPin,
{
    /// Create a new Half-H bridge.
    ///
    /// This function creates a new Half-H bridge with the given input and enable pin.
    ///
    /// # Note
    ///
    /// It will *not* enable the output or otherwise initialize the circuit. Thus, the
    /// initial state of the Half-H bridge depends on the states of the input and enable pins
    /// given.
    ///
    /// # Examples
    ///
    /// ```
    /// use l293x::HalfH;
    ///
    /// // [...] create the input and enable pins
    /// let mut bridge = HalfH::new(input, enable);
    /// ```
    #[inline]
    pub fn new(input: I, enable: E) -> Self {
        Self { input, enable }
    }

    /// Enable the output of the Half-H bridge.
    ///
    /// This will set the enable pin of the [HalfH] bridge high and thus, enables the output
    /// of the bridge. After calling this method, the output will either be high or low, depending
    /// on the state of the input pin.
    ///
    /// # Errors
    ///
    /// This method will return the error defined by the given enable pin, in case of an error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use l293x::HalfH;
    /// # let mut bridge = HalfH::new(input, enable);
    /// bridge.enable().unwrap();
    /// ```
    #[inline]
    pub fn enable(&mut self) -> Result<(), E::Error> {
        self.enable.set_high()
    }

    /// Disable the output of the Half-H bridge.
    ///
    /// This will set the enable pin of the [HalfH] bridge low and thus, disables the output
    /// of the bridge. After calling this method, the output will be in high impedance mode and
    /// the electrical level of the output depends on the components connected to it.
    ///
    /// # Errors
    ///
    /// This method will return the error defined by the given enable pin, in case of an error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use l293x::HalfH;
    /// # let mut bridge = HalfH::new(input, enable);
    /// bridge.disable().unwrap();
    /// ```
    #[inline]
    pub fn disable(&mut self) -> Result<(), E::Error> {
        self.enable.set_low()
    }
}
impl<I, E> HalfH<I, E>
where
    E: StatefulOutputPin,
{
    /// Checks whether the output of the [HalfH] is enabled.
    ///
    /// # Note
    /// This method does *not* check for the electrical level of the Half-H bridge, but uses an
    /// internal state of the enable pin instead. Thus, the actual electrical state of the enable
    /// pin of the Half-H bridge might be different, depending on the circuit layout.
    ///
    /// # Errors
    ///
    /// Returns the error defined by the enable pin in case of an error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use l293x::HalfH;
    /// # let mut bridge = HalfH::new(input, enable);
    /// match bridge.is_enabled().unwrap() {
    ///     true => println!("The Half-H bridge output is enabled");
    ///     false => println!("The Half-H bridge output is disabled");
    /// }
    /// ```
    #[inline]
    pub fn is_enabled(&mut self) -> Result<bool, E::Error> {
        self.enable.is_set_high()
    }

    /// Checks whether the output of the [HalfH] is disabled.
    ///
    /// # Note
    /// This method does *not* check for the electrical level of the Half-H bridge, but uses an
    /// internal state of the enable pin instead. Thus, the actual electrical state of the enable
    /// pin of the Half-H bridge might be different, depending on the circuit layout.
    ///
    /// # Errors
    ///
    /// Returns the error defined by the enable pin in case of an error.
    ///
    /// # Examples
    ///
    /// ```
    /// # use l293x::HalfH;
    /// # let mut bridge = HalfH::new(input, enable);
    /// match bridge.is_disabled().unwrap() {
    ///     true => println!("The Half-H bridge output is disabled");
    ///     false => println!("The Half-H bridge output is enabled");
    /// }
    /// ```
    #[inline]
    pub fn is_disabled(&mut self) -> Result<bool, E::Error> {
        self.enable.is_set_low()
    }
}

impl<I, E> embedded_hal::digital::ErrorType for HalfH<I, E>
where
    I: ErrorType,
    E: ErrorType,
{
    type Error = OutputStateError<I::Error, E::Error>;
}

impl<I, E> OutputPin for HalfH<I, E>
where
    I: OutputPin,
    E: ErrorType,
{
    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.input
            .set_low()
            .map_err(OutputStateError::InputPinError)
    }

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.input
            .set_high()
            .map_err(OutputStateError::InputPinError)
    }
}

impl<I, E> StatefulOutputPin for HalfH<I, E>
where
    I: StatefulOutputPin,
    E: StatefulOutputPin,
{
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        match self
            .is_enabled()
            .map_err(OutputStateError::EnablePinError)?
        {
            false => Ok(false),
            true => Ok(self
                .input
                .is_set_high()
                .map_err(OutputStateError::InputPinError)?),
        }
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        match self
            .is_enabled()
            .map_err(OutputStateError::EnablePinError)?
        {
            false => Ok(false),
            true => Ok(self
                .input
                .is_set_low()
                .map_err(OutputStateError::InputPinError)?),
        }
    }
}

impl<I, E> embedded_hal::pwm::ErrorType for HalfH<I, E>
where
    I: SetDutyCycle,
{
    type Error = I::Error;
}

impl<I, E> embedded_hal::pwm::SetDutyCycle for HalfH<I, E>
where
    I: SetDutyCycle,
{
    fn max_duty_cycle(&self) -> u16 {
        self.input.max_duty_cycle()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.input.set_duty_cycle(duty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{DigitalError, DigitalPin, PwmPin};

    fn bridge() -> HalfH<DigitalPin, DigitalPin> {
        HalfH::new(DigitalPin::new(), DigitalPin::new())
    }

    #[test]
    fn check_output_state_error_kind() {
        let input_error: OutputStateError<DigitalError, DigitalError> =
            OutputStateError::InputPinError(DigitalError());
        assert_eq!(input_error.kind(), DigitalError().kind());

        let enable_error: OutputStateError<DigitalError, DigitalError> =
            OutputStateError::EnablePinError(DigitalError());
        assert_eq!(enable_error.kind(), DigitalError().kind());
    }

    #[test]
    fn check_output_state_error_equality() {
        let i: OutputStateError<DigitalError, DigitalError> =
            OutputStateError::InputPinError(DigitalError());
        let e: OutputStateError<DigitalError, DigitalError> =
            OutputStateError::EnablePinError(DigitalError());

        assert_eq!(i, i);
        assert_eq!(e, e);
        assert_ne!(e, i);
        assert_ne!(i, e);
    }

    #[test]
    fn enable_disable() {
        let mut b = bridge();

        b.enable().unwrap();
        assert!(b.enable.is_set_high().unwrap());
        assert!(b.is_enabled().unwrap());
        assert!(!b.is_disabled().unwrap());

        b.disable().unwrap();
        assert!(b.enable.is_set_low().unwrap());
        assert!(!b.is_enabled().unwrap());
        assert!(b.is_disabled().unwrap());
    }

    #[test]
    fn set_state() {
        let mut b = bridge();

        // LOW, LOW => Z
        b.set_low().unwrap();
        b.disable().unwrap();
        assert!(!b.is_set_high().unwrap());
        assert!(!b.is_set_low().unwrap());

        // HIGH, LOW => Z
        b.set_high().unwrap();
        b.disable().unwrap();
        assert!(!b.is_set_high().unwrap());
        assert!(!b.is_set_low().unwrap());

        // LOW, HiGH => LOW
        b.set_low().unwrap();
        b.enable().unwrap();
        assert!(!b.is_set_high().unwrap());
        assert!(b.is_set_low().unwrap());

        // HIGH, HiGH => HIGH
        b.set_high().unwrap();
        b.enable().unwrap();
        assert!(b.is_set_high().unwrap());
        assert!(!b.is_set_low().unwrap());
    }

    #[test]
    fn test_set_state_error() {
        let mut b = HalfH::new(DigitalPin::new(), DigitalPin::new());
        b.input.fail();

        assert_eq!(
            b.set_low().unwrap_err(),
            OutputStateError::InputPinError(DigitalError())
        );
        assert_eq!(
            b.set_high().unwrap_err(),
            OutputStateError::InputPinError(DigitalError())
        );
    }

    #[test]
    fn test_check_state_input_error() {
        let mut b = HalfH::new(DigitalPin::new(), DigitalPin::new());
        b.enable().unwrap();
        b.input.fail();

        assert_eq!(
            b.is_set_low().unwrap_err(),
            OutputStateError::InputPinError(DigitalError())
        );
        assert_eq!(
            b.is_set_high().unwrap_err(),
            OutputStateError::InputPinError(DigitalError())
        );
    }

    #[test]
    fn test_check_state_enable_error() {
        let mut b = HalfH::new(DigitalPin::new(), DigitalPin::new());
        b.enable.fail();

        assert_eq!(
            b.is_set_low().unwrap_err(),
            OutputStateError::EnablePinError(DigitalError())
        );
        assert_eq!(
            b.is_set_high().unwrap_err(),
            OutputStateError::EnablePinError(DigitalError())
        );
    }

    #[test]
    fn max_duty_cycle() {
        let b = HalfH::new(PwmPin::new(), DigitalPin::new());
        assert_eq!(b.input.max_duty_cycle(), b.max_duty_cycle());
    }

    #[test]
    fn set_duty_cycle() {
        let duty = u16::MAX;
        let mut b = HalfH::new(PwmPin::new(), DigitalPin::new());

        b.set_duty_cycle(duty).unwrap();
        assert_eq!(duty, b.input.get_duty_cycle());
    }
}
