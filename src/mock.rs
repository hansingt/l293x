extern crate alloc;

use core::convert::Infallible;
use embedded_hal::digital::{ErrorKind, ErrorType, OutputPin, StatefulOutputPin};

#[derive(Debug)]
pub struct DigitalPin {
    state: bool,
    should_fail: bool,
}

impl DigitalPin {
    pub fn new() -> Self {
        Self {
            state: false,
            should_fail: false,
        }
    }

    pub fn fail(&mut self) {
        self.should_fail = true
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
        match self.should_fail {
            false => {
                self.state = false;
                Ok(())
            }
            true => Err(DigitalError()),
        }
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        match self.should_fail {
            false => {
                self.state = true;
                Ok(())
            }
            true => Err(DigitalError()),
        }
    }
}

impl StatefulOutputPin for DigitalPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        match self.should_fail {
            false => Ok(self.state),
            true => Err(DigitalError()),
        }
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        match self.should_fail {
            false => Ok(!self.state),
            true => Err(DigitalError()),
        }
    }
}

#[derive(Debug)]
pub struct PwmPin(u16);
impl PwmPin {
    #[inline]
    pub fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub fn get_duty_cycle(&self) -> u16 {
        self.0
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
        self.0 = duty;
        Ok(())
    }
}
