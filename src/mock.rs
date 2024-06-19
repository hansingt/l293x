use embedded_hal::{digital, pwm};

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

impl digital::Error for DigitalError {
    #[inline]
    fn kind(&self) -> digital::ErrorKind {
        digital::ErrorKind::Other
    }
}

impl digital::ErrorType for DigitalPin {
    type Error = DigitalError;
}

impl digital::OutputPin for DigitalPin {
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

impl digital::StatefulOutputPin for DigitalPin {
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
pub struct PwmPin {
    duty: u16,
    should_fail: bool,
}

impl PwmPin {
    #[inline]
    pub fn new() -> Self {
        Self {
            duty: 0,
            should_fail: false,
        }
    }

    #[inline]
    pub fn get_duty_cycle(&self) -> u16 {
        self.duty
    }

    pub fn fail(&mut self) {
        self.should_fail = true;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PwmError();

impl pwm::Error for PwmError {
    #[inline]
    fn kind(&self) -> pwm::ErrorKind {
        pwm::ErrorKind::Other
    }
}

impl pwm::ErrorType for PwmPin {
    type Error = PwmError;
}

impl pwm::SetDutyCycle for PwmPin {
    fn max_duty_cycle(&self) -> u16 {
        u16::MAX
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        match self.should_fail {
            true => Err(PwmError()),
            false => {
                self.duty = duty;
                Ok(())
            }
        }
    }
}
