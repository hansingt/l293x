use embedded_hal::digital::{Error, ErrorKind};

/// Error returned by the [L293x](crate::L293x) and [HalfH](crate::bridge::HalfH) implementations.
///
/// This enumeration combines the possible errors returned by the input pin and the enable pin.
///
/// Depending on the source of the error, either an [InputPinError](OutputStateError::InputPinError)
/// or a [EnablePinError](OutputStateError::EnablePinError) will be returned by the functions
/// implemented in the [OutputPin](embedded_hal::digital::OutputPin) traits.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock::DigitalError;

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
}