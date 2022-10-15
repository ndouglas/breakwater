use crate::astronomy::moon::error::Error as MoonError;

/// Moon-related errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Moon Error.
  MoonError(MoonError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    MoonError(moon_error) => format!("an error occurred in the moon ({})", honeyholt_brief!(moon_error)),
  }
});

impl From<MoonError> for Error {
  #[named]
  fn from(error: MoonError) -> Self {
    Error::MoonError(error)
  }
}
