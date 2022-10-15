use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_subsystem::error::Error as StarSubsystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// StarSubsystem Error.
  StarSubsystemError(StarSubsystemError),
  /// No suitable StarSubsystems found.
  NoSuitableSubsystemsCouldBeGenerated,
}

impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}

impl From<StarSubsystemError> for Error {
  #[named]
  fn from(error: StarSubsystemError) -> Self {
    Error::StarSubsystemError(error)
  }
}
