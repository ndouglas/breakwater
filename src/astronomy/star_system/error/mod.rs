use std::convert::From;

use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_system::subsystem::error::Error as SubsystemError;

/// Star system errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// Subsystem Error.
  SubsystemError(SubsystemError),
}

impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}

impl From<SubsystemError> for Error {
  #[named]
  fn from(error: SubsystemError) -> Self {
    Error::SubsystemError(error)
  }
}
