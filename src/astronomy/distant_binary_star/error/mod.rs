use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;
// use crate::astronomy::star_subsystem::error::Error as StarSubsystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Planetary System Error
  PlanetarySystemError(PlanetarySystemError),
  // Star Subsystem Error
  // StarSubsystemError(StarSubsystemError),
}

impl From<PlanetarySystemError> for Error {
  #[named]
  fn from(error: PlanetarySystemError) -> Self {
    Error::PlanetarySystemError(error)
  }
}

// impl From<StarSubsystemError> for Error {
//   #[named]
//   fn from(error: StarSubsystemError) -> Self {
//     Error::StarSubsystemError(error)
//   }
// }

