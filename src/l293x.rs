use crate::bridge::{HalfH, OutputStateError};
use crate::shared_pin::SharedPin;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use embedded_hal::pwm::SetDutyCycle;

/// L293 or L293D chip driver
///
/// This struct allows controlling the four [Half-H bridges](crate::bridge::HalfH),
/// of the chip. These bridges share two enable pins
/// (one for bridge 1 & 2 and one for 3 & 4). So that two of them can be
/// enabled or disabled together. The output of each of them can be controlled
/// using the four input pins.
///
/// These pins can either be [digital pins](embedded_hal::digital::OutputPin)
/// or [PWM pins](embedded_hal::pwm::SetDutyCycle). This allows to use the
/// L293x chips as a motor driver for two motors. Each driver takes two of the
/// Half-H-Bridges and thus, can be enabled or disabled, using a single pin.
/// The direction and drive speed can then be controlled using the two input
/// pins of the Half-H-Bridges.
///
/// For more information, please see the [crate documentation](crate).
///
/// # Examples
///
/// A new L293x driver can be created using the [`new()`](L293x::new) method:
///
/// ```
/// use l293x::L293x;
/// // [...] create the input pins
/// let mut l293x = L293x::new(input1, input2, input3, input4, enable12, enable34);
/// ```
///
/// And then the different bridges can be enabled or disabled:
///
/// ```
/// # use l293x::L293x;
/// # let mut l293x = L293x::new(input1, input2, input3, input4, enable12, enable34);
/// l293x.enable12().unwrap();
/// assert!(l293x.is_output12_enabled().unwrap());
///
/// l293x.disable34().unwrap();
/// assert!(l293x.is_output34_disabled().unwrap());
/// ```
#[derive(Debug)]
pub struct L293x<I1, I2, I3, I4, EN12, EN34>
where
    EN12: OutputPin,
    EN34: OutputPin,
{
    /// Half-H bridge controlling output y1
    pub y1: HalfH<I1, SharedPin<EN12>>,
    /// Half-H bridge controlling output y2
    pub y2: HalfH<I2, SharedPin<EN12>>,
    /// Half-H bridge controlling output y3
    pub y3: HalfH<I3, SharedPin<EN34>>,
    /// Half-H bridge controlling output y4
    pub y4: HalfH<I4, SharedPin<EN34>>,
}

impl<I1, I2, I3, I4, EN12, EN34> L293x<I1, I2, I3, I4, EN12, EN34>
where
    EN12: OutputPin,
    EN34: OutputPin,
{
    /// Create a new L293x chip driver.
    ///
    /// This function takes the given input and enable pins and constructs
    /// a L293x chip driver from them.
    ///
    /// # Examples
    ///
    /// ```
    /// # use l293x::L293x;
    /// // [...] create the input pins
    /// let l293x = L293x::new(input1, input2, input3, input4, enable12, enable34);
    /// ```
    pub fn new(a1: I1, a2: I2, a3: I3, a4: I4, en12: EN12, en34: EN34) -> Self {
        let en12 = SharedPin::new(en12);
        let en34 = SharedPin::new(en34);
        Self {
            y1: HalfH::new(a1, en12.clone()),
            y2: HalfH::new(a2, en12),
            y3: HalfH::new(a3, en34.clone()),
            y4: HalfH::new(a4, en34),
        }
    }

    /// Enable the output channels 1 & 2.
    ///
    /// This method enables the Half-H-Bridges 1 & 2 which share a common
    /// enable pin. This then allows the inputs 1 & 2 to control the levels
    /// of the output pins 1 & 2.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 1 & 2. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn enable_y1_and_y2(&mut self) -> Result<(), EN12::Error> {
        self.y1.enable()
    }

    /// Disable the output channels 1 & 2.
    ///
    /// This method disables the Half-H-Bridges 1 & 2 which share a common
    /// enable pin. This forces the output channels 1 & 2 to a low level,
    /// independent of the level of the corresponding inputs.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 1 & 2. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn disable_y1_and_y2(&mut self) -> Result<(), EN12::Error> {
        self.y1.disable()
    }

    /// Enable the output channels 3 & 4.
    ///
    /// This method enables the Half-H-Bridges 3 & 4 which share a common
    /// enable pin. This then allows the inputs 3 & 4 to control the levels
    /// of the output pins 3 & 4.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 3 & 4. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn enable_y3_and_y4(&mut self) -> Result<(), EN34::Error> {
        self.y3.enable()
    }

    /// Disable the output channels 3 & 4.
    ///
    /// This method disables the Half-H-Bridges 3 & 4 which share a common
    /// enable pin. This forces the output channels 3 & 4 to a low level,
    /// independent of the level of the corresponding inputs.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 3 & 4. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    pub fn disable_y3_and_y4(&mut self) -> Result<(), EN34::Error> {
        self.y3.disable()
    }
}

impl<I1, I2, I3, I4, EN12, EN34> L293x<I1, I2, I3, I4, EN12, EN34>
where
    EN12: StatefulOutputPin,
    EN34: OutputPin,
{
    /// Check whether the output channels 1 & 2 are enabled.
    ///
    /// This method checks whether the Half-H-Bridges 1 & 2 are enabled.
    /// These bridges need to be enabled to allow the corresponding inputs to
    /// control the output levels.
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 1 & 2. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    ///
    /// # Examples
    ///
    /// ```
    /// l293x.enable_y1_and_y2().unwrap();
    /// assert!(l293x.y1_and_y2_enabled().unwrap());
    /// ```
    pub fn y1_and_y2_enabled(&mut self) -> Result<bool, EN12::Error> {
        self.y1.is_enabled()
    }

    /// Check whether the output channels 1 & 2 are disabled.
    ///
    /// This method checks whether the Half-H-Bridges 1 & 2 are disabled.
    /// These bridges need to be enabled to allow the corresponding inputs to
    /// control the output levels.
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 1 & 2. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    ///
    /// # Examples
    ///
    /// ```
    /// l293x.disable_y1_and_y2().unwrap();
    /// assert!(l293x.y1_and_y2_disabled().unwrap());
    /// ```
    pub fn y1_and_y2_disabled(&mut self) -> Result<bool, EN12::Error> {
        self.y1.is_disabled()
    }
}

impl<I1, I2, I3, I4, EN12, EN34> L293x<I1, I2, I3, I4, EN12, EN34>
where
    EN12: OutputPin,
    EN34: StatefulOutputPin,
{
    /// Check whether the output channels 3 & 4 are enabled.
    ///
    /// This method checks whether the Half-H-Bridges 3 & 4 are enabled.
    /// These bridges need to be enabled to allow the corresponding inputs to
    /// control the output levels.
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 3 & 4. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    ///
    /// # Examples
    ///
    /// ```
    /// l293x.enable_y3_and_y4().unwrap();
    /// assert!(l293x.y3_and_y4_enabled().unwrap());
    /// ```
    pub fn y3_and_y4_enabled(&mut self) -> Result<bool, EN34::Error> {
        self.y3.is_enabled()
    }

    /// Check whether the output channels 3 & 4 are disabled.
    ///
    /// This method checks whether the Half-H-Bridges 3 & 4 are disabled.
    /// These bridges need to be enabled to allow the corresponding inputs to
    /// control the output levels.
    ///
    /// # Note
    ///
    /// Please note, that this method does not check the electrical level of
    /// the pin, but uses an internal state instead. The electrical level of
    /// the pin may vary due to the layout of the circuit.
    ///
    /// # Errors
    ///
    /// This function will return the error of the common enable pin
    /// of the Half-H-Bridges 3 & 4. The concrete error type returned depends
    /// on the [OutputPin](embedded_hal::digital::OutputPin) used.
    ///
    /// # Examples
    ///
    /// ```
    /// l293x.disable_y3_and_y4().unwrap();
    /// assert!(l293x.y3_and_y4_disabled().unwrap());
    /// ```
    pub fn y3_and_y4_disabled(&mut self) -> Result<bool, EN34::Error> {
        self.y3.is_disabled()
    }
}

macro_rules! output_pin_impl {
    ($output:ident, $type_:ty, $enable:ty) => {
        paste::item! {
            impl<I1, I2, I3, I4, EN12, EN34> L293x<I1, I2, I3, I4, EN12, EN34>
            where
                EN12: OutputPin,
                EN34: OutputPin,
                $type_: OutputPin,
            {
                #[doc = "Set the output " $output " high"]
                ///
                /// # Note
                ///
                #[doc = "This function sets the input of the output channel " $output]
                /// to high.
                /// For the output to actually become "high", the corresponding
                /// output channel needs to be enabled as well either using the
                /// [L293x::enable_y1_and_y2()] or the [L293x::enable_y3_and_y4()]
                /// method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it
                ///
                /// # Errors
                ///
                /// In case of an error, while setting the input "high", this method
                /// returns the error of the corresponding input pin. The actual type
                /// of error returned depends on the type of the input pin used.
                pub fn [< set_ $output _high >](
                    &mut self
                ) -> Result<(), OutputStateError<$type_::Error, $enable::Error>> {
                    self.$output.set_high()
                }

                #[doc = "Set the output " $output " low"]
                ///
                /// # Note
                ///
                #[doc = "This function sets the input of the output channel " $output]
                /// to low.
                /// For the output to actually become "low", the corresponding
                /// output channel needs to be enabled as well either using the
                /// [L293x::enable_y1_and_y2()] or the [L293x::enable_y3_and_y4()]
                /// method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it
                ///
                /// # Errors
                ///
                /// In case of an error, while setting the input "low", this method
                /// returns the error of the corresponding input pin. The actual type
                /// of error returned depends on the type of the input pin used.
                pub fn [< set_ $output _low >](
                    &mut self
                ) -> Result<(), OutputStateError<$type_::Error, $enable::Error>> {
                    self.$output.set_low()
                }
                #[doc = "Set the state of output " $output]
                ///
                /// # Note
                ///
                /// This function sets state of the input for output channel
                #[doc = $output "."]
                /// For the output to actually take the given state, the corresponding
                /// output channel needs to be enabled as well either using the
                /// [L293x::enable_y1_and_y2()] or the [L293x::enable_y3_and_y4()]
                /// method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it
                ///
                /// # Errors
                ///
                /// In case of an error, while setting the input state, this method
                /// returns the error of the corresponding input pin. The actual type
                /// of error returned depends on the type of the input pin used.
                pub fn [< set_ $output _state >](
                    &mut self,
                    state: embedded_hal::digital::PinState
                ) -> Result<(), OutputStateError<$type_::Error, $enable::Error>> {
                    self.$output.set_state(state)
                }
            }
        }
    };
}
output_pin_impl!(y1, I1, EN12);
output_pin_impl!(y2, I2, EN12);
output_pin_impl!(y3, I3, EN34);
output_pin_impl!(y4, I4, EN34);

macro_rules! stateful_output_pin_impl {
    ($output:ident, $type_:ty, $enable:ty) => {
        paste::item! {
            impl<I1, I2, I3, I4, EN12, EN34> L293x<I1, I2, I3, I4, EN12, EN34>
            where
                $type_: StatefulOutputPin,
                EN12: embedded_hal::digital::OutputPin,
                EN34: embedded_hal::digital::OutputPin,
                $enable: embedded_hal::digital::StatefulOutputPin,
            {
                #[doc = "Check if output " $output " is high"]
                ///
                /// The output of a L293x chip is high, only the output is enabled and
                /// the if the corresponding input is high.
                ///
                /// If the output is disabled, it is neither high, nor low but remains
                /// in an high impendance state und thus, its electrical level depends
                /// on the components connected to it.
                ///
                /// # Note
                ///
                /// Please note, that this method does not check the electrical level of
                /// the pin, but uses an internal state instead. The electrical level of
                /// the pin may vary due to the layout of the circuit.
                ///
                /// # Errors
                ///
                /// In case of an error, while reading the state of the enable pin,
                /// this method will return an [OutputStateError::EnablePinError]
                /// with the actual error of the enable pin returned.
                ///
                /// If an error occurs while reading the state of the input pin, an
                /// [OutputStateError::InputPinError] with the error of the input pin will be
                /// returned instead.
                pub fn [< is_ $output _high >](
                    &mut self
                ) -> Result<bool, OutputStateError<$type_::Error, $enable::Error>> {
                    self.$output.is_set_high()
                }

                #[doc = "Check if output " $output " is low"]
                ///
                /// The output of a L293x chip is low, only the output is enabled and
                /// the if the corresponding input is low.
                ///
                /// If the output is disabled, it is neither high, nor low but remains
                /// in an high impendance state und thus, its electrical level depends
                /// on the components connected to it.
                ///
                /// # Note
                ///
                /// Please note, that this method does not check the electrical level of
                /// the pin, but uses an internal state instead. The electrical level of
                /// the pin may vary due to the layout of the circuit.
                ///
                /// # Errors
                ///
                /// In case of an error, while reading the state of the enable pin, this method
                /// will return an [OutputStateError::EnablePinError]
                /// with the actual error of the enable pin returned.
                ///
                /// If an error occurs while reading the state of the input pin, an
                /// [OutputStateError::InputPinError] with the error of the input pin will be
                /// returned instead.
                pub fn [< is_ $output _low >](
                    &mut self
                ) -> Result<bool, OutputStateError<$type_::Error, $enable::Error>> {
                    self.$output.is_set_low()
                }

                #[doc = "Toggle the state of output " $output]
                ///
                /// If the pin is considered high, it will be set to low and vice versa.
                ///
                /// # Note
                ///
                /// This method toggles the state of the corresponding input channel,
                /// but does *not* enable the output channel, if it is disabled,
                /// to avoid side effects. Thus, for the output
                /// to actually toggle, the corresponding output needs to be enabled
                /// using either the [L293x::enable_y1_and_y2()] or
                /// [L293x::enable_y3_and_y4()] method.
                ///
                /// # Errors
                ///
                /// If an error occurs while toggling the state of the input pin, the
                /// error of the input pin will be returned. The actual type of error
                /// depends on the type of input pin used.
                pub fn [< toggle_ $output >](
                    &mut self
                ) -> Result<(), OutputStateError<$type_::Error, $enable::Error>> {
                    self.$output.toggle()
                }
            }
        }
    };
}
stateful_output_pin_impl!(y1, I1, EN12);
stateful_output_pin_impl!(y2, I2, EN12);
stateful_output_pin_impl!(y3, I3, EN34);
stateful_output_pin_impl!(y4, I4, EN34);

macro_rules! pwm_pin_impl {
    ($output:ident, $type_:ty) => {
        paste::item! {
            impl<I1, I2, I3, I4, EN12, EN34> L293x<I1, I2, I3, I4, EN12, EN34>
            where
                EN12: embedded_hal::digital::OutputPin,
                EN34: embedded_hal::digital::OutputPin,
                $type_: SetDutyCycle,
            {
                #[doc = "Get the max duty value of output " $output]
                ///
                /// This method returns the maximum value, that can be used in the
                #[doc = "[L293x::set_" $output "_duty_cycle()]"]
                /// method.
                ///
                /// # Examples
                ///
                /// ```
                #[doc = "let max_duty = l293x." $output "_max_duty_cycle();"]
                #[doc = "l293x.set_" $output "_duty_cycle(max_duty).unwrap();"]
                /// ```
                pub fn [< $output _max_duty_cycle >](&self) -> u16 {
                    self.$output.max_duty_cycle()
                }

                #[doc = "Set the duty cycle of output " $output]
                ///
                /// This method sets the duty cycle of output channel
                #[doc =  $output "."]
                /// The duty cycle describes the portion of the interval that the
                /// output should be set "active", which actually might either mean
                /// high or low. This depends on the configuration of the PWM input
                /// pin used.
                ///
                /// The level of activity scales linearly between `0` and the
                /// maximum duty cycle value returned by the
                #[doc = "[L293x::" $output "_max_duty_cycle()] method."]
                ///
                /// # Note
                ///
                /// Please note, that this function only set the duty cycle of the
                /// input channel. For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the
                /// [L293x::enable_y1_and_y2()] or [L293x::enable_y3_and_y4()] method.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                ///
                /// # Examples
                ///
                /// To set the PWM pin inactive, set the duty cycle to `0`:
                /// ```
                #[doc = "l293x.set_" $output "_duty_cycle(0).unwrap();"]
                /// ```
                ///
                /// To make the pin always active, set it to the max duty value
                /// returned by the
                #[doc = "[L293x::" $output "_max_duty_cycle()] method:"]
                /// ```
                #[doc = "let max_duty = l293x." $output "_max_duty_cycle();"]
                #[doc = "l293x.set_" $output "_duty_cycle(max_duty).unwrap();"]
                /// ```
                pub fn [< set_ $output _duty_cycle >](
                    &mut self, duty: u16
                ) -> Result<(), $type_::Error> {
                    self.$output.set_duty_cycle(duty)
                }

                #[doc = "Set the duty cycle of output " $output " by fraction."]
                ///
                /// This method sets the duty cycle of the output channel
                #[doc = $output "by a fraction defined using the given `nom` and"]
                /// `denom` parameters.
                ///
                /// This method is useful if you want to set the output duty cycle
                /// depending on an input value that can be between `0` and a
                /// maximum value.
                ///
                /// For this, the fraction must be in between `0` and `1`, this means
                /// that the `denom` must not be `0` and the `nom` must be smaller or
                /// equal to `denom`.
                ///
                /// # Note
                ///
                /// Please note, that this function only set the duty cycle of the
                /// input channel. For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the [L293x::enable_y1_and_y2()] or
                /// [L293x::enable_y3_and_y4()] method.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                ///
                /// # Examples
                ///
                /// To set the duty cycle of the output channel depending on another
                /// input value, set the `denom` to the maximum value possible and the
                /// `nom` to the current value:
                ///
                /// ```
                /// // `max_value` defines the maximum value of the `input`.
                #[doc = "l293x.set_" $output "_duty_cycle_fraction(input, max_value).unwrap();"]
                /// ```
                pub fn [< set_ $output _duty_cycle_fraction >](
                    &mut self, nom: u16, denom: u16
                ) -> Result<(), $type_::Error> {
                    self.$output.set_duty_cycle_fraction(nom, denom)
                }

                #[doc = "Set the duty cycle of output " $output " by percent"]
                ///
                #[doc = "This method sets the output " $output " active for the"]
                /// given percent of its PWM interval.
                ///
                /// The `percent` value must be between `0` and `100` (inclusive),
                /// where `0` means fully off, and `100` means fully on.
                ///
                /// # Note
                ///
                /// Please note, that this function only set the duty cycle of the
                /// input channel. For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the [L293x::enable_y1_and_y2()] or
                /// [L293x::enable_y3_and_y4()] method.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                pub fn [< set_ $output _duty_cycle_percent >](
                    &mut self, percent: u8,
                ) -> Result<(), $type_::Error> {
                    self.$output.set_duty_cycle_percent(percent)
                }

                #[doc = "Fully enable the output " $output]
                ///
                #[doc = "This method set the PWM output channel " $output]
                /// fully active.
                ///
                /// # Note
                ///
                /// Please note, that this function only set the duty cycle of the
                /// input channel. For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the [L293x::enable_y1_and_y2()] or
                /// [L293x::enable_y3_and_y4()] method.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                pub fn [< set_ $output _duty_cycle_fully_on >](
                    &mut self
                ) -> Result<(), $type_::Error> {
                    self.$output.set_duty_cycle_fully_on()
                }

                #[doc = "Fully disable the output " $output]
                ///
                #[doc = "This method set the PWM output channel " $output]
                /// fully inactive.
                ///
                /// # Note
                ///
                /// Please note, that this function only set the duty cycle of the
                /// input channel. For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the [L293x::enable_y1_and_y2()] or
                /// [L293x::enable_y3_and_y4()] method.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                pub fn [< set_ $output _duty_cycle_fully_off >](
                    &mut self
                ) -> Result<(), $type_::Error> {
                    self.$output.set_duty_cycle_fully_off()
                }
            }
        }
    };
}
pwm_pin_impl!(y1, I1);
pwm_pin_impl!(y2, I2);
pwm_pin_impl!(y3, I3);
pwm_pin_impl!(y4, I4);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::{DigitalPin, PwmPin};
    use embedded_hal::digital::{PinState, StatefulOutputPin};

    #[test]
    fn test_enable12() {
        let mut l293x = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );

        l293x.enable_y1_and_y2().unwrap();
        assert!(l293x.y1_and_y2_enabled().unwrap());
        assert!(!l293x.y1_and_y2_disabled().unwrap());

        l293x.disable_y1_and_y2().unwrap();
        assert!(!l293x.y1_and_y2_enabled().unwrap());
        assert!(l293x.y1_and_y2_disabled().unwrap());
    }

    #[test]
    fn test_enable34() {
        let mut l293x = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );

        l293x.enable_y3_and_y4().unwrap();
        assert!(l293x.y3_and_y4_enabled().unwrap());
        assert!(!l293x.y3_and_y4_disabled().unwrap());

        l293x.disable_y3_and_y4().unwrap();
        assert!(!l293x.y3_and_y4_enabled().unwrap());
        assert!(l293x.y3_and_y4_disabled().unwrap());
    }

    #[test]
    fn test_split() {
        let l293x = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );
        let mut l293x_2 = L293x::new(
            l293x.y1,
            l293x.y2,
            l293x.y3,
            l293x.y4,
            DigitalPin::new(),
            DigitalPin::new(),
        );
        l293x_2.set_y1_high().unwrap();
        assert!(l293x_2.y1.input.input.is_set_high().unwrap())
    }

    macro_rules! test_output {
        ($name:ident, $bname:ident) => {
            paste::item! {
                #[test]
                fn [< test_ $name >]() {
                    let mut l293x = L293x::new(
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                    );

                    // LOW, LOW => Z
                    l293x.[< set_ $name _low >]().unwrap();
                    l293x.[< disable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _high >]().unwrap());
                    assert!(!l293x.[< is_ $name _low >]().unwrap());

                    // HIGH, LOW => Z
                    l293x.[< set_ $name _high >]().unwrap();
                    l293x.[< disable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _high >]().unwrap());
                    assert!(!l293x.[< is_ $name _low >]().unwrap());

                    // LOW, HIGH => LOW
                    l293x.[< set_ $name _low >]().unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _high >]().unwrap());
                    assert!(l293x.[< is_ $name _low >]().unwrap());

                    // HIGH, HIGH => HIGH
                    l293x.[< set_ $name _high >]().unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(l293x.[< is_ $name _high >]().unwrap());
                    assert!(!l293x.[< is_ $name _low >]().unwrap());
                }

                #[test]
                fn [< test_ $name _set_state >]() {
                    let mut l293x = L293x::new(
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                    );

                    // LOW, LOW => Z
                    l293x.[< set_ $name _state >](PinState::Low).unwrap();
                    l293x.[< disable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _high >]().unwrap());
                    assert!(!l293x.[< is_ $name _low >]().unwrap());

                    // HIGH, LOW => Z
                    l293x.[< set_ $name _state >](PinState::High).unwrap();
                    l293x.[< disable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _high >]().unwrap());
                    assert!(!l293x.[< is_ $name _low >]().unwrap());

                    // LOW, HIGH => LOW
                    l293x.[< set_ $name _state >](PinState::Low).unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _high >]().unwrap());
                    assert!(l293x.[< is_ $name _low >]().unwrap());

                    // HIGH, HIGH => HIGH
                    l293x.[< set_ $name _state >](PinState::High).unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(l293x.[< is_ $name _high >]().unwrap());
                    assert!(!l293x.[< is_ $name _low >]().unwrap());
                }

                #[test]
                fn [< test_toggle_ $name >]() {
                    let mut l293x = L293x::new(
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                    );
                    l293x.[< set_ $name _state >](PinState::Low).unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    let old_state = l293x.[< is_ $name _high >]().unwrap();

                    l293x.[< toggle_ $name >]().unwrap();
                    assert_ne!(l293x.[< is_ $name _high >]().unwrap(), old_state);

                    l293x.[< toggle_ $name >]().unwrap();
                    assert_eq!(l293x.[< is_ $name _high >]().unwrap(), old_state);
                }

                #[test]
                fn [< test_ $name _pwm >]() {
                    let mut l293x = L293x::new(
                        PwmPin::new(),
                        PwmPin::new(),
                        PwmPin::new(),
                        PwmPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                    );

                    let max_duty = l293x.[< $name _max_duty_cycle >]();

                    l293x.[< set_ $name _ duty_cycle >](max_duty).unwrap();
                    assert_eq!(l293x.$name.input.get_duty_cycle(), max_duty);

                    l293x.[< set_ $name _ duty_cycle_fraction >](1, 2).unwrap();
                    assert_eq!(l293x.$name.input.get_duty_cycle(), max_duty / 2);

                    l293x.[< set_ $name _ duty_cycle_percent >](25).unwrap();
                    assert_eq!(l293x.$name.input.get_duty_cycle(), max_duty / 4);

                    l293x.[< set_ $name _ duty_cycle_fully_on >]().unwrap();
                    assert_eq!(l293x.$name.input.get_duty_cycle(), max_duty);

                    l293x.[< set_ $name _ duty_cycle_fully_off >]().unwrap();
                    assert_eq!(l293x.$name.input.get_duty_cycle(), 0);
                }
            }
        };
    }
    test_output!(y1, y1_and_y2);
    test_output!(y2, y1_and_y2);
    test_output!(y3, y3_and_y4);
    test_output!(y4, y3_and_y4);
}
