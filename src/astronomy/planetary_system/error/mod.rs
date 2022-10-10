use std::convert::From;

use crate::astronomy::host_star::error::Error as HostStarError;
// use crate::astronomy::planetary_system::error::Error as PlanetarySystemError;

/// Star system errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  HostStarError(HostStarError),
}

impl From<HostStarError> for Error {
  #[named]
  fn from(error: HostStarError) -> Self {
    Error::HostStarError(error)
  }
}

// impl From<PlanetarySystemError> for Error {
//   #[named]
//   fn from(error: PlanetarySystemError) -> Self {
//     Error::PlanetarySystemError(error)
//   }
// }
