extern crate alloc;

use alloc::rc::Rc;
use core::cell::Cell;
use core::convert::Infallible;
use embedded_hal::digital::{ErrorKind, ErrorType, OutputPin, StatefulOutputPin};

#[derive(Debug, Clone)]
pub struct DigitalPin {
    state: Rc<Cell<bool>>,
    should_fail: Rc<Cell<bool>>,
}

impl DigitalPin {
    pub fn new() -> Self {
        Self {
            state: Rc::new(Cell::new(false)),
            should_fail: Rc::new(Cell::new(false)),
        }
    }

    pub fn fail(&mut self) {
        self.should_fail.set(true)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct DigitalError();

impl embedded_hal::digital::Error for DigitalError {
    #[inline]
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl ErrorType for DigitalPin {
    type Error = DigitalError;
}

impl OutputPin for DigitalPin {
    fn set_low(&mut self) -> Result<(), Self::Error> {
        match self.should_fail.get() {
            false => {
                self.state.set(false);
                Ok(())
            }
            true => Err(DigitalError()),
        }
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        match self.should_fail.get() {
            false => {
                self.state.set(true);
                Ok(())
            }
            true => Err(DigitalError()),
        }
    }
}

impl StatefulOutputPin for DigitalPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        match self.should_fail.get() {
            false => Ok(self.state.get()),
            true => Err(DigitalError()),
        }
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        match self.should_fail.get() {
            false => Ok(!self.state.get()),
            true => Err(DigitalError()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PwmPin(Rc<Cell<u16>>);

impl PwmPin {
    #[inline]
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(0)))
    }

    #[inline]
    pub fn get_duty_cycle(&self) -> u16 {
        self.0.get()
    }
}

impl embedded_hal::pwm::ErrorType for PwmPin {
    type Error = Infallible;
}

impl embedded_hal::pwm::SetDutyCycle for PwmPin {
    fn max_duty_cycle(&self) -> u16 {
        u16::MAX
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.0.set(duty);
        Ok(())
    }
}
