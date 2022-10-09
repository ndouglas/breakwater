use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_subsystem::error::Error as SubsystemError;

/// Binary star subsystem configuration errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// Star Subsystem Error.
  SubsystemError(Box<SubsystemError>),
  /// The habitable zone is contained within the forbidden zone.
  HabitableZoneContainedWithinForbiddenZone,
  /// The habitable zone isn't sufficiently far from the host stars.
  HabitableZoneContainedWithinDangerZone,
  /// No habitable conditions found anywhere in subsystem.
  NoHabitableZoneFound,
  /// Lower than MINIMUM_BINARY_STAR_SEPARATION.
  BinaryStarsTooCloseForComfort,
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
    Error::SubsystemError(Box::new(error))
  }
}
