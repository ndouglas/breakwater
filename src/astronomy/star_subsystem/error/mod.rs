use std::convert::From;

use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_subsystem::binary_configuration::error::Error as BinaryConfigurationError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// Binary configuration error.
  BinaryConfigurationError(BinaryConfigurationError),
  /// Unable to generate binary subsystem after a number of tries.
  UnableToGenerateBinaryConfiguration,
}

/// Allow seamlessly converting Star errors into Star System errors.
impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}

impl From<BinaryConfigurationError> for Error {
  #[named]
  fn from(error: BinaryConfigurationError) -> Self {
    Error::BinaryConfigurationError(error)
  }
}
