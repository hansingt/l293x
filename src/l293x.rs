use core::cell::RefCell;
use core::fmt::Debug;

use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use embedded_hal::pwm::SetDutyCycle;

use crate::HalfH;

/// L293 or L293D chip driver
///
/// This struct allows controlling the four Half-H bridges of the chip. These bridges share two
/// enable pins (one for bridge 1 & 2 and one for 3 & 4). So that two of them can be
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
///
/// ## Basic Usage
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
/// assert!(l293x.is_output12_enabled()?);
///
/// l293x.disable34().unwrap();
/// assert!(l293x.is_output34_disabled()?);
/// ```
///
/// ## Partial Usage
///
/// The four Half-H bridges of the L293 Chip can be use independent of each other. Because of this,
/// if you only want to use parts of the chip, you could pass the empty type (`()`) instead of a
/// real pin for the inputs not connected:
///
/// ```
/// # use l293x::L293x;
/// let mut l293x = L293x::new(input1, (), (), (), enable12, ());
/// ```
///
/// This causes the type to only implement the functions for the matching outputs.
///
/// ```compile_fail
/// # use l293x::L293x;
/// # let mut l293x = L293x::new(input1, (), (), (), enable12, ());
/// l293x.set_y1_high()?;  // <-- Ok!
/// l293x.set_y2_high()?;  // <-- Does not compile!
/// ```
///
/// Because parts of the functionalities of the output relies on the enable pins
/// to be connected, leaving the enable pins not connected does not work. Instead, if your enable
/// pin is always connected to Vcc, the [Vcc](crate::pins::Vcc) struct can be used to express this:
///
/// ```
/// # use l293x::L293x;
/// use l293x::pins::Vcc;
///
/// let mut l293x = L293x::new(input1, (), (), (), Vcc(), ());
/// ```
///
/// This will signal that the corresponding pin is always high and cannot be set low. For more
/// information please see the documentation of the [pins](crate::pins) module.
///
/// ## Splitting the chip
///
/// The outputs of the L293 chip can be used as inputs to other components. Thus, to be able
/// to express this, this type implements [`y1()`](L293x::y1) - [`y4()`](L293x::y4) methods. They
/// take a reference to the chip and return a [HalfH], which can be used to control one of the four
/// Half-H bridges of the chip.
///
/// <div class="warning">
/// Please keep in mind, that the Half-H bridges of the L293 chip share common enable pins, and
/// thus, the bridges 1 & 2 and the bridges 3 & 4 can only be enabled or disables together!
/// </div>
///
/// ```
/// # use l293x::L293x;
/// # let mut l293x = L293x::new(input1, input2, input3, input4, enable12, enable34);
/// let mut split = l293x.split();
///
/// split.y1().enable()?;
/// split.y1().set_high()?;
/// ```

#[derive(Debug)]
pub struct L293x<A1, A2, A3, A4, EN12, EN34> {
    a1: RefCell<A1>,
    a2: RefCell<A2>,
    a3: RefCell<A3>,
    a4: RefCell<A4>,
    en12: RefCell<EN12>,
    en34: RefCell<EN34>,
}

impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34> {
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
    #[inline]
    pub fn new(a1: A1, a2: A2, a3: A3, a4: A4, en12: EN12, en34: EN34) -> Self {
        Self {
            a1: RefCell::new(a1),
            a2: RefCell::new(a2),
            a3: RefCell::new(a3),
            a4: RefCell::new(a4),
            en12: RefCell::new(en12),
            en34: RefCell::new(en34),
        }
    }

    #[inline]
    pub fn y1(&self) -> HalfH<A1, EN12> {
        HalfH::new(&self.a1, &self.en12)
    }

    #[inline]
    pub fn y2(&self) -> HalfH<A2, EN12> {
        HalfH::new(&self.a2, &self.en12)
    }

    #[inline]
    pub fn y3(&self) -> HalfH<A3, EN34> {
        HalfH::new(&self.a3, &self.en34)
    }

    #[inline]
    pub fn y4(&self) -> HalfH<A4, EN34> {
        HalfH::new(&self.a4, &self.en34)
    }
}

impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
where
    EN12: OutputPin,
{
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
        self.en12.get_mut().set_high()
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
        self.en12.get_mut().set_low()
    }
}

impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
where
    EN34: OutputPin,
{
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
        self.en34.get_mut().set_high()
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
        self.en34.get_mut().set_low()
    }
}

impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
where
    EN12: StatefulOutputPin,
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
        self.en12.get_mut().is_set_high()
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
        self.en12.get_mut().is_set_low()
    }
}

impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
where
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
        self.en34.get_mut().is_set_high()
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
        self.en34.get_mut().is_set_low()
    }
}

macro_rules! output_pin_impl {
    ($output:ident, $input:ident, $type_:ty) => {
        paste::item! {
            impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
            where
                $type_: OutputPin,
            {
                #[doc = "Set the output " $output " high"]
                ///
                /// # Note
                ///
                #[doc = "This function sets the input of the output channel " $output]
                /// to high.
                ///
                /// In contrast to the [`set_high`](HalfH::set_high) method of the
                /// [`$output()`](L293x::$output), this method does **not** enable the output.
                ///
                /// For the output to actually become "high", the corresponding
                /// output channel needs to be enabled as well either using the
                /// [L293x::enable_y1_and_y2()] or the [L293x::enable_y3_and_y4()]
                /// method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
                ///
                /// # Errors
                ///
                /// In case of an error, while setting the input "high", this method
                /// returns the error of the corresponding input pin. The actual type
                /// of error returned depends on the type of the input pin used.
                pub fn [< set_ $output _high >](
                    &mut self
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_high()
                }

                #[doc = "Set the output " $output " low"]
                ///
                /// # Note
                ///
                #[doc = "This function sets the input of the output channel " $output]
                /// to low.
                ///
                /// In contrast to the [`set_low`](HalfH::set_high) method of the
                /// [`$output()`](L293x::$output), this method does **not** enable the output.
                ///
                /// For the output to actually become "low", the corresponding
                /// output channel needs to be enabled as well either using the
                /// [L293x::enable_y1_and_y2()] or the [L293x::enable_y3_and_y4()]
                /// method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
                ///
                /// # Errors
                ///
                /// In case of an error, while setting the input "low", this method
                /// returns the error of the corresponding input pin. The actual type
                /// of error returned depends on the type of the input pin used.
                pub fn [< set_ $output _low >](
                    &mut self
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_low()
                }

                #[doc = "Set the state of output " $output]
                ///
                /// # Note
                ///
                /// This function sets state of the input for output channel
                #[doc = $output "."]
                ///
                /// In contrast to the [`set_state`](HalfH::set_state) method of the
                /// [`$output()`](L293x::$output), this method does **not** enable the output.
                ///
                /// For the output to actually take the given state, the corresponding
                /// output channel needs to be enabled as well either using the
                /// [L293x::enable_y1_and_y2()] or the [L293x::enable_y3_and_y4()]
                /// method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
                ///
                /// # Errors
                ///
                /// In case of an error, while setting the input state, this method
                /// returns the error of the corresponding input pin. The actual type
                /// of error returned depends on the type of the input pin used.
                pub fn [< set_ $output _state >](
                    &mut self,
                    state: embedded_hal::digital::PinState
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_state(state)
                }
            }
        }
    };
}
output_pin_impl!(y1, a1, A1);
output_pin_impl!(y2, a2, A2);
output_pin_impl!(y3, a3, A3);
output_pin_impl!(y4, a4, A4);

macro_rules! stateful_output_pin_impl {
    ($output:ident, $type_:ty, $enable_ty:ty) => {
        paste::item! {
            impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
            where
                $type_: StatefulOutputPin,
                $enable_ty: StatefulOutputPin,
            {
                #[doc = "Check if output " $output " is set high"]
                ///
                /// The output of a L293x chip is high, only the output is enabled and
                /// the if the corresponding input is high.
                ///
                /// # Note
                ///
                /// Please note, that this method does not check the electrical level of
                /// the pin, but uses an internal state instead. The electrical level of
                /// the pin may vary due to the layout of the circuit.
                ///
                /// # Errors
                ///
                /// If the output is disabled, it is neither high, nor low but remains
                /// in an high impendance state und thus, its electrical level depends
                /// on the components connected to it. In this case, a
                /// [OutputStateError::NotEnabled] error will be returned.
                ///
                /// In case of an error, while reading the state of the enable pin,
                /// this method will return an [OutputStateError::EnablePinError]
                /// with the actual error of the enable pin returned.
                ///
                /// If an error occurs while reading the state of the input pin, an
                /// [OutputStateError::InputPinError] with the error of the input pin will be
                /// returned instead.
                pub fn [< is_ $output _set_high >](
                    &mut self
                ) -> Result<bool, <crate::half_h::HalfH<$type_, $enable_ty> as embedded_hal::digital::ErrorType>::Error> {
                    self.$output().is_set_high()
                }

                #[doc = "Check if output " $output " is set low"]
                ///
                /// The output of a L293x chip is low, only the output is enabled and
                /// the if the corresponding input is low.
                ///
                /// # Note
                ///
                /// Please note, that this method does not check the electrical level of
                /// the pin, but uses an internal state instead. The electrical level of
                /// the pin may vary due to the layout of the circuit.
                ///
                /// # Errors
                ///
                /// If the output is disabled, it is neither high, nor low but remains
                /// in an high impendance state und thus, its electrical level depends
                /// on the components connected to it. In this case, a
                /// [OutputStateError::NotEnabled] error will be returned.
                ///
                /// In case of an error, while reading the state of the enable pin, this method
                /// will return an [OutputStateError::EnablePinError]
                /// with the actual error of the enable pin returned.
                ///
                /// If an error occurs while reading the state of the input pin, an
                /// [OutputStateError::InputPinError] with the error of the input pin will be
                /// returned instead.
                pub fn [< is_ $output _set_low >](
                    &mut self
                ) -> Result<bool, <crate::half_h::HalfH<$type_, $enable_ty> as embedded_hal::digital::ErrorType>::Error> {
                    self.$output().is_set_low()
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
                /// If the output is disabled, it is neither high, nor low but remains
                /// in an high impendance state und thus, its electrical level depends
                /// on the components connected to it. In this case, the output cannot be toggled
                /// and a [OutputStateError::NotEnabled] error will be returned.
                ///
                /// If an error occurs while toggling the state of the input pin, the
                /// error of the input pin will be returned. The actual type of error
                /// depends on the type of input pin used.
                pub fn [< toggle_ $output >](
                    &mut self
                ) -> Result<(), <crate::half_h::HalfH<$type_, $enable_ty> as embedded_hal::digital::ErrorType>::Error> {
                    self.$output().toggle()
                }
            }
        }
    };
}
stateful_output_pin_impl!(y1, A1, EN12);
stateful_output_pin_impl!(y2, A2, EN12);
stateful_output_pin_impl!(y3, A3, EN34);
stateful_output_pin_impl!(y4, A4, EN34);

macro_rules! pwm_pin_impl {
    ($output:ident, $input:ident, $type_:ty) => {
        paste::item! {
            impl<A1, A2, A3, A4, EN12, EN34> L293x<A1, A2, A3, A4, EN12, EN34>
            where
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
                    self.$input.borrow().max_duty_cycle()
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
                /// In contrast to the [`set_duty_cycle`](HalfH::set_duty_cycle) method of the
                /// [`$output()`](L293x::$output), this method does **not** enable the output.
                ///
                /// For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the
                /// [L293x::enable_y1_and_y2()] or [L293x::enable_y3_and_y4()] method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
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
                    self.$input.get_mut().set_duty_cycle(duty)
                }

                #[doc = "Set the duty cycle of output " $output " by fraction."]
                ///
                /// This method sets the duty cycle of the output channel
                #[doc = $output "by a fraction defined using the given `num` and"]
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
                /// In contrast to the [`set_duty_cycle_fraction`](HalfH::set_duty_cycle_fraction)
                /// method of the [`$output()`](L293x::$output), this method does **not** enable
                /// the output.
                ///
                /// For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the
                /// [L293x::enable_y1_and_y2()] or [L293x::enable_y3_and_y4()] method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
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
                    &mut self, num: u16, denom: u16
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_duty_cycle_fraction(num, denom)
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
                /// In contrast to the [`set_duty_cycle_percent`](HalfH::set_duty_cycle_percent)
                /// method of the [`$output()`](L293x::$output), this method does **not** enable
                /// the output.
                ///
                /// For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the
                /// [L293x::enable_y1_and_y2()] or [L293x::enable_y3_and_y4()] method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                pub fn [< set_ $output _duty_cycle_percent >](
                    &mut self, percent: u8,
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_duty_cycle_percent(percent)
                }

                #[doc = "Fully enable the output " $output]
                ///
                #[doc = "This method set the PWM output channel " $output]
                /// fully active.
                ///
                /// # Note
                ///
                /// In contrast to the [`set_duty_cycle_fully_on`](HalfH::set_duty_cycle_fully_on)
                /// method of the [`$output()`](L293x::$output), this method does **not** enable
                /// the output.
                ///
                /// For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the
                /// [L293x::enable_y1_and_y2()] or [L293x::enable_y3_and_y4()] method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                pub fn [< set_ $output _duty_cycle_fully_on >](
                    &mut self
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_duty_cycle_fully_on()
                }

                #[doc = "Fully disable the output " $output]
                ///
                #[doc = "This method set the PWM output channel " $output]
                /// fully inactive.
                ///
                /// # Note
                ///
                /// In contrast to the [`set_duty_cycle_fully_off`](HalfH::set_duty_cycle_fully_off)
                /// method of the [`$output()`](L293x::$output), this method does **not** enable
                /// the output.
                ///
                /// For the output to actually become active for the
                /// same amount of time, the corresponding output channel needs to be
                /// enabled as well using either the
                /// [L293x::enable_y1_and_y2()] or [L293x::enable_y3_and_y4()] method.
                ///
                /// If the channel is disabled, the output will remain in high
                /// impendance state and the state of the output is depending on the
                /// components connected to it.
                ///
                /// This is useful, if the states of the two outputs sharing an enable pin need
                /// to change states together i.e. to avoid high currencies in circuit.
                /// In this case, the outputs can be disabled first, the new states of the outputs
                /// set and afterwards, both outputs can be enabled again together.
                ///
                /// # Errors
                ///
                /// This method will return the error of the input pin, in case of an
                /// error while setting the duty cycle of the pin. The actual type of
                /// error returned depends on the type of input pin used.
                pub fn [< set_ $output _duty_cycle_fully_off >](
                    &mut self
                ) -> Result<(), $type_::Error> {
                    self.$input.get_mut().set_duty_cycle_fully_off()
                }
            }
        }
    };
}
pwm_pin_impl!(y1, a1, A1);
pwm_pin_impl!(y2, a2, A2);
pwm_pin_impl!(y3, a3, A3);
pwm_pin_impl!(y4, a4, A4);

#[cfg(test)]
mod tests {
    use coverage_helper::test;
    use embedded_hal::digital::PinState;

    use crate::mock::{DigitalError, DigitalPin, PwmPin};
    use crate::pins::Vcc;
    use crate::OutputStateError;

    use super::*;

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
    fn test_enable12_fail() {
        let mut l293x = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );
        l293x.en12.get_mut().fail();
        assert!(matches!(
            l293x.is_y1_set_high().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
        assert!(matches!(
            l293x.is_y2_set_high().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
        assert!(matches!(
            l293x.is_y1_set_low().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
        assert!(matches!(
            l293x.is_y2_set_low().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
    }

    #[test]
    fn test_enable34_fail() {
        let mut l293x = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );
        l293x.en34.get_mut().fail();
        assert!(matches!(
            l293x.is_y3_set_high().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
        assert!(matches!(
            l293x.is_y4_set_high().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
        assert!(matches!(
            l293x.is_y3_set_low().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
        assert!(matches!(
            l293x.is_y4_set_low().unwrap_err(),
            OutputStateError::EnablePinError(..)
        ));
    }

    macro_rules! test_output {
        ($name:ident, $input:ident, $bname:ident) => {
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
                    assert!(matches!(l293x.[< is_ $name _set_high >](), Err(OutputStateError::NotEnabled)));
                    assert!(matches!(l293x.[< is_ $name _set_low >](), Err(OutputStateError::NotEnabled)));

                    // HIGH, LOW => Z
                    l293x.[< set_ $name _high >]().unwrap();
                    l293x.[< disable_ $bname >]().unwrap();
                    assert!(matches!(l293x.[< is_ $name _set_high >](), Err(OutputStateError::NotEnabled)));
                    assert!(matches!(l293x.[< is_ $name _set_low >](), Err(OutputStateError::NotEnabled)));

                    // LOW, HIGH => LOW
                    l293x.[< set_ $name _low >]().unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _set_high >]().unwrap());
                    assert!(l293x.[< is_ $name _set_low >]().unwrap());

                    // HIGH, HIGH => HIGH
                    l293x.[< set_ $name _high >]().unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(l293x.[< is_ $name _set_high >]().unwrap());
                    assert!(!l293x.[< is_ $name _set_low >]().unwrap());
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
                    assert!(matches!(l293x.[< is_ $name _set_high >](), Err(OutputStateError::NotEnabled)));
                    assert!(matches!(l293x.[< is_ $name _set_low >](), Err(OutputStateError::NotEnabled)));

                    // HIGH, LOW => Z
                    l293x.[< set_ $name _state >](PinState::High).unwrap();
                    l293x.[< disable_ $bname >]().unwrap();
                    assert!(matches!(l293x.[< is_ $name _set_high >](), Err(OutputStateError::NotEnabled)));
                    assert!(matches!(l293x.[< is_ $name _set_low >](), Err(OutputStateError::NotEnabled)));

                    // LOW, HIGH => LOW
                    l293x.[< set_ $name _state >](PinState::Low).unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(!l293x.[< is_ $name _set_high >]().unwrap());
                    assert!(l293x.[< is_ $name _set_low >]().unwrap());

                    // HIGH, HIGH => HIGH
                    l293x.[< set_ $name _state >](PinState::High).unwrap();
                    l293x.[< enable_ $bname >]().unwrap();
                    assert!(l293x.[< is_ $name _set_high >]().unwrap());
                    assert!(!l293x.[< is_ $name _set_low >]().unwrap());
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
                    let old_state = l293x.[< is_ $name _set_high >]().unwrap();

                    l293x.[< toggle_ $name >]().unwrap();
                    assert_ne!(l293x.[< is_ $name _set_high >]().unwrap(), old_state);

                    l293x.[< toggle_ $name >]().unwrap();
                    assert_eq!(l293x.[< is_ $name _set_high >]().unwrap(), old_state);
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

                    l293x.[< set_ $name _duty_cycle >](max_duty).unwrap();
                    assert_eq!(l293x.$input.borrow().get_duty_cycle(), max_duty);

                    l293x.[< set_ $name _duty_cycle_fraction >](1, 2).unwrap();
                    assert_eq!(l293x.$input.borrow().get_duty_cycle(), max_duty / 2);

                    l293x.[< set_ $name _duty_cycle_percent >](25).unwrap();
                    assert_eq!(l293x.$input.borrow().get_duty_cycle(), max_duty / 4);

                    l293x.[< set_ $name _duty_cycle_fully_on >]().unwrap();
                    assert_eq!(l293x.$input.borrow().get_duty_cycle(), max_duty);

                    l293x.[< set_ $name _duty_cycle_fully_off >]().unwrap();
                    assert_eq!(l293x.$input.borrow().get_duty_cycle(), 0);
                }
            }
        };
    }
    test_output!(y1, a1, y1_and_y2);
    test_output!(y2, a2, y1_and_y2);
    test_output!(y3, a3, y3_and_y4);
    test_output!(y4, a4, y3_and_y4);

    macro_rules! test_fail {
        ($name:ident, $input:ident) => {
            paste::item! {
                #[test]
                fn [< test_ $name _fail >]() {
                    let mut l293x = L293x::new(
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                        DigitalPin::new(),
                    );
                    l293x.enable_y1_and_y2().unwrap();
                    l293x.enable_y3_and_y4().unwrap();
                    l293x.$input.get_mut().fail();

                    assert!(matches!(l293x.[< set_ $name _low >](), Err(DigitalError(..))));
                    assert!(matches!(l293x.[< set_ $name _high >](), Err(DigitalError(..))));
                    assert!(matches!(l293x.[< is_ $name _set_high >](), Err(OutputStateError::InputPinError(..))));
                    assert!(matches!(l293x.[< is_ $name _set_low >](), Err(OutputStateError::InputPinError(..))));
                    assert!(matches!(l293x.[< toggle_ $name >](), Err(OutputStateError::InputPinError(..))));
                }
            }
        };
    }
    test_fail!(y1, a1);
    test_fail!(y2, a2);
    test_fail!(y3, a3);
    test_fail!(y4, a4);

    #[test]
    fn test_partial_chip() {
        let mut l293x_part = L293x::new(DigitalPin::new(), (), (), (), Vcc(), ());

        l293x_part.set_y1_high().unwrap();
        assert!(l293x_part.is_y1_set_high().unwrap())
    }

    #[test]
    fn test_split() {
        let split = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );
        let mut y1 = split.y1();
        let mut y1_cpy = split.y1();

        y1.enable().unwrap();
        y1.set_high().unwrap();
        assert!(y1_cpy.is_set_high().unwrap())
    }

    #[test]
    fn test_cascading_l293() {
        let split = L293x::new(
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
            DigitalPin::new(),
        );
        let mut l293x = L293x::new(split.y1(), split.y2(), split.y3(), split.y4(), Vcc(), Vcc());
        split.y1().enable().unwrap();
        l293x.set_y1_high().unwrap();
        assert!(split.y1().is_set_high().unwrap());
    }
}
