use crate::astronomy::stellar_neighbor::error::Error as StellarNeighborError;

/// Stellar Neighborhood errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Stellar Neighbor Error.
  StellarNeighborError(StellarNeighborError),
}

impl From<StellarNeighborError> for Error {
  #[named]
  fn from(error: StellarNeighborError) -> Self {
    Error::StellarNeighborError(error)
  }
}
