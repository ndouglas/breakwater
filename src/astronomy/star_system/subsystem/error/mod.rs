use std::convert::From;

use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_system::subsystem::binary_configuration::error::Error as BinaryConfigurationError;

/// Star system errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// Binary configuration error.
  BinaryConfigurationError(BinaryConfigurationError),
  /// Unable to generate binary subsystem after a number of tries.
  UnableToGenerateBinaryConfiguration,
  /// The habitable zone is contained within the forbidden zone.
  HabitableZoneContainedWithinForbiddenZone,
  /// The habitable zone isn't sufficiently far from the host stars.
  HabitableZoneTooCloseToBinaryHostStars,
  /// No habitable conditions found anywhere in subsystem.
  NoHabitableZoneFound,
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
