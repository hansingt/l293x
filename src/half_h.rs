use core::cell::RefCell;

use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use embedded_hal::pwm::SetDutyCycle;

use crate::OutputStateError;

/// Half-H bridge of the [L293](crate::L293x) chip
///
/// This is returned by the [`y1()`](crate::L293x::y1) - [`y4()`](crate::L293x::y4) methods of the
/// [L293](crate::L293x) chip driver and can be used to control a single output of the chip.
///
/// They can be used as inputs for other drivers based on the [embedded_hal] traits. Because of
/// this, they implement the [embedded_hal::digital::OutputPin],
/// [embedded_hal::digital::StatefulOutputPin], or the [embedded_hal::pwm::SetDutyCycle] traits,
/// based on the traits implemented by the corresponding input. Output `y1` is linked with input
/// `a1`, `y2` with `a2` and so on.
///
/// <div class="warning">
/// Please keep in mind, that the four Half-H bridges of the L293 chip share two common enable pins.
/// Thus, the bridges 1 & 2 and the bridges 3 & 4 can only be enabled or disabled together.
///
/// This means, that enabling or disabling an output returned by the
/// [`y1()`](crate::L293x::y1) - [`y4()`](crate::L293x::y4) methods will **always** enable or disable
/// a second output as well!
/// </div>
///
/// # Examples
///
/// ```
/// # use embedded_hal::digital::OutputPin;
/// use l293x::{L293x, pins::Vcc};
/// // Do some things, to initialize the input pin of the L293 chip
/// let l293 = L293x::new(input, (), (), (), Vcc(), ());
/// let mut y1 = l293.y1();
/// // Now, you can use y1, to control the output `y1` of the chip.
/// y1.enable()?;
/// y1.set_high()?;
/// ```
#[derive(Debug, Copy, Clone)]
pub struct HalfH<'a, INPUT, ENABLE> {
    input: &'a RefCell<INPUT>,
    enable: &'a RefCell<ENABLE>,
}

impl<'a, INPUT, ENABLE> HalfH<'a, INPUT, ENABLE> {
    pub(crate) fn new(input: &'a RefCell<INPUT>, enable: &'a RefCell<ENABLE>) -> Self {
        Self { input, enable }
    }
}

impl<'a, INPUT, ENABLE> HalfH<'a, INPUT, ENABLE>
where
    ENABLE: OutputPin,
{
    /// Enable the output
    ///
    /// This method sets the output from the high impedance mode into the enabled mode.
    /// After calling this method, the output is either set to high or low, depending on the
    /// state of the input pin.
    ///
    /// # Errors
    ///
    /// If the output cannot be enabled, this method will return the error returned by the
    /// enable pin to the caller. The concrete error type returned depends on the type of
    /// [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn enable(&mut self) -> Result<(), ENABLE::Error> {
        self.enable.borrow_mut().set_high()
    }

    /// Disable the output
    ///
    /// This method sets the output into the high impedance mode.
    /// After calling this method, the output is disconnected from the input and always remains
    /// in high impedance mode. The actual electrical level of the pin depends on the components
    /// it is connected to in the circuit.
    ///
    /// # Errors
    ///
    /// If the output cannot be disabled, this method will return the error returned by the
    /// enable pin to the caller. The concrete error type returned depends on the type of
    /// [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn disable(&mut self) -> Result<(), ENABLE::Error> {
        self.enable.borrow_mut().set_low()
    }
}

impl<'a, INPUT, ENABLE> HalfH<'a, INPUT, ENABLE>
where
    ENABLE: StatefulOutputPin,
{
    /// Check whether the output is enabled.
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// If the output cannot be enabled, this method will return the error returned by the
    /// enable pin to the caller. The concrete error type returned depends on the type of
    /// [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn is_enabled(&mut self) -> Result<bool, ENABLE::Error> {
        self.enable.borrow_mut().is_set_high()
    }

    /// Check whether the output is disabled.
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// If the output cannot be disabled, this method will return the error returned by the
    /// enable pin to the caller. The concrete error type returned depends on the type of
    /// [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn is_disabled(&mut self) -> Result<bool, ENABLE::Error> {
        self.enable.borrow_mut().is_set_low()
    }
}

impl<'a, INPUT, ENABLE> embedded_hal::digital::ErrorType for HalfH<'a, INPUT, ENABLE>
where
    INPUT: OutputPin,
    ENABLE: OutputPin,
{
    type Error = OutputStateError<INPUT::Error, ENABLE::Error>;
}

impl<'a, INPUT, ENABLE> OutputPin for HalfH<'a, INPUT, ENABLE>
where
    INPUT: OutputPin,
    ENABLE: OutputPin,
{
    /// Set the output of the bridge to low state
    ///
    /// This method will enable the bridge, if it is not enabled yet.
    ///
    /// # Errors
    ///
    /// If an error occurs while enabling the output, a
    /// [EnablePinError](OutputStateError::EnablePinError) will be returned.
    ///
    /// If an error occurs while setting the state of the input pin, a
    /// [InputPinError](OutputStateError::InputPinError) will be returned.
    ///
    /// Both of them contain the original error to provide additional information to the caller.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.enable().map_err(OutputStateError::EnablePinError)?;
        self.input
            .borrow_mut()
            .set_low()
            .map_err(OutputStateError::InputPinError)
    }

    /// Set the output of the bridge to low state
    ///
    /// This method will enable the bridge, if it is not enabled yet.
    ///
    /// # Errors
    ///
    /// If an error occurs while enabling the output, a
    /// [EnablePinError](OutputStateError::EnablePinError) will be returned.
    ///
    /// If an error occurs while setting the state of the input pin, a
    /// [InputPinError](OutputStateError::InputPinError) will be returned.
    ///
    /// Both of them contain the original error to provide additional information to the caller.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.enable().map_err(OutputStateError::EnablePinError)?;
        self.input
            .borrow_mut()
            .set_high()
            .map_err(OutputStateError::InputPinError)
    }
}

impl<'a, INPUT, ENABLE> StatefulOutputPin for HalfH<'a, INPUT, ENABLE>
where
    INPUT: StatefulOutputPin,
    ENABLE: StatefulOutputPin,
{
    /// Check whether the output is set high
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// If the bridge is not [enabled](HalfH::enable), this method will return a
    /// [OutputStateError::NotEnabled] error.
    ///
    /// Otherwise, if an error occurs, while checking the state of the enable pin, a
    /// [EnablePinError](OutputStateError::EnablePinError) will be returned, or if an error occurs
    /// while checking the state of the input pin, a
    /// [InputPinError](OutputStateError::InputPinError) will be returned instead.
    ///
    /// Both of them contain the original error to provide additional information to the caller.
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        match self
            .enable
            .borrow_mut()
            .is_set_high()
            .map_err(OutputStateError::EnablePinError)?
        {
            false => Err(OutputStateError::NotEnabled),
            true => self
                .input
                .borrow_mut()
                .is_set_high()
                .map_err(OutputStateError::InputPinError),
        }
    }

    /// Check whether the output is set low
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// If the bridge is not [enabled](HalfH::enable), this method will return a
    /// [OutputStateError::NotEnabled] error.
    ///
    /// Otherwise, if an error occurs, while checking the state of the enable pin, a
    /// [EnablePinError](OutputStateError::EnablePinError) will be returned, or if an error occurs
    /// while checking the state of the input pin, a
    /// [InputPinError](OutputStateError::InputPinError) will be returned instead.
    ///
    /// Both of them contain the original error to provide additional information to the caller.
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        match self
            .enable
            .borrow_mut()
            .is_set_high()
            .map_err(OutputStateError::EnablePinError)?
        {
            false => Err(OutputStateError::NotEnabled),
            true => self
                .input
                .borrow_mut()
                .is_set_low()
                .map_err(OutputStateError::InputPinError),
        }
    }

    /// Toggle the state of the output
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state to determine the current output state.
    /// The electrical level of the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// If the bridge is not [enabled](HalfH::enable), this method will return a
    /// [OutputStateError::NotEnabled] error.
    ///
    /// Otherwise, if an error occurs, while checking the state of the enable pin, a
    /// [EnablePinError](OutputStateError::EnablePinError) will be returned, or if an error occurs
    /// while checking the state of the input pin, a
    /// [InputPinError](OutputStateError::InputPinError) will be returned instead.
    ///
    /// Both of them contain the original error to provide additional information to the caller.
    fn toggle(&mut self) -> Result<(), Self::Error> {
        match self
            .enable
            .borrow_mut()
            .is_set_high()
            .map_err(OutputStateError::EnablePinError)?
        {
            false => Err(OutputStateError::NotEnabled),
            true => self
                .input
                .borrow_mut()
                .toggle()
                .map_err(OutputStateError::InputPinError),
        }
    }
}

impl<'a, INPUT, ENABLE> embedded_hal::pwm::ErrorType for HalfH<'a, INPUT, ENABLE>
where
    INPUT: SetDutyCycle,
{
    type Error = INPUT::Error;
}

impl<'a, INPUT, ENABLE> SetDutyCycle for HalfH<'a, INPUT, ENABLE>
where
    INPUT: SetDutyCycle,
{
    fn max_duty_cycle(&self) -> u16 {
        self.input.borrow_mut().max_duty_cycle()
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.input.borrow_mut().set_duty_cycle(duty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{DigitalPin, PwmPin};
    use crate::pins::Vcc;
    use crate::L293x;
    use coverage_helper::test;

    fn l293() -> L293x<DigitalPin, DigitalPin, DigitalPin, DigitalPin, DigitalPin, DigitalPin> {
        L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        )
    }

    #[test]
    fn test_enable() {
        let l293 = l293();
        let mut bridge = l293.y1();

        bridge.disable().unwrap();
        assert!(!bridge.is_enabled().unwrap());
        assert!(bridge.is_disabled().unwrap());

        bridge.enable().unwrap();
        assert!(bridge.is_enabled().unwrap());
        assert!(!bridge.is_disabled().unwrap());
    }

    #[test]
    fn test_set() {
        let l293 = l293();
        let mut bridge = l293.y1();
        bridge.set_low().unwrap();
        assert!(!bridge.is_set_high().unwrap());
        assert!(bridge.is_set_low().unwrap());

        bridge.set_high().unwrap();
        assert!(bridge.is_set_high().unwrap());
        assert!(!bridge.is_set_low().unwrap());
    }

    #[test]
    fn test_set_enables_output() {
        let l293 = l293();
        let mut bridge = l293.y1();

        assert!(!bridge.is_enabled().unwrap());
        bridge.set_high().unwrap();
        assert!(bridge.is_enabled().unwrap());

        bridge.disable().unwrap();
        bridge.set_low().unwrap();
        assert!(bridge.is_enabled().unwrap());
    }

    #[test]
    fn test_set_error() {
        let l293 = l293();
        let mut bridge = l293.y1();
        bridge.enable.borrow_mut().fail();

        assert!(matches!(bridge.set_high(), Err(OutputStateError::EnablePinError(..))));
        assert!(matches!(bridge.set_low(), Err(OutputStateError::EnablePinError(..))));
    }

    #[test]
    fn test_toggle() {
        let l293 = l293();
        let mut bridge = l293.y1();

        bridge.disable().unwrap();
        assert!(matches!(bridge.toggle(), Err(OutputStateError::NotEnabled)));

        bridge.enable().unwrap();
        let old_state = bridge.is_set_high().unwrap();
        assert!(matches!(bridge.toggle(), Ok(())));
        assert_ne!(bridge.is_set_high().unwrap(), old_state);
    }

    #[test]
    fn test_toggle_error() {
        let mut enable = DigitalPin::new();
        let l293 = L293x::new(DigitalPin::new(), (), (), (), enable.clone(), ());
        let mut bridge = l293.y1();

        enable.fail();
        assert!(matches!(
            bridge.toggle(),
            Err(OutputStateError::EnablePinError(..))
        ));
    }

    #[test]
    fn test_pwm() {
        let pin = PwmPin::new();
        let l293 = L293x::new(pin.clone(), (), (), (), Vcc(), ());
        let mut bridge = l293.y1();
        let max_duty = bridge.max_duty_cycle();

        for duty in [max_duty, max_duty / 2, 0] {
            bridge.set_duty_cycle(duty).unwrap();
            assert_eq!(pin.get_duty_cycle(), duty);
        }
    }
}
