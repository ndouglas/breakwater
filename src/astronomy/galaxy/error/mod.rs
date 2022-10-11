use crate::astronomy::stellar_neighborhood::error::Error as StellarNeighborhoodError;

/// Galaxy-class errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Stellar Neighborhood Error.
  StellarNeighborhoodError(StellarNeighborhoodError),
}

impl From<StellarNeighborhoodError> for Error {
  #[named]
  fn from(error: StellarNeighborhoodError) -> Self {
    Error::StellarNeighborhoodError(error)
  }
}
