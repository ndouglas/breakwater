use super::orbit::error::Error as OrbitError;

/// Star-related errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Encountered an error calculating orbits.
  OrbitError(OrbitError),
  /// Lower than MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND.
  MassTooLowForMainSequence,
  /// Higher than MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND.
  MassTooHighForMainSequence,
  /// Lower than MINIMUM_STAR_AGE_TO_SUPPORT_LIFE.
  TooYoungToSupportLife,
  /// Lower than MINIMUM_STAR_MASS_TO_SUPPORT_LIFE.
  MassTooLowToSupportLife,
  /// Higher than MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE.
  MassTooHighToSupportLife,
}

impl From<OrbitError> for Error {
  #[named]
  fn from(error: OrbitError) -> Self {
    Error::OrbitError(error)
  }
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    OrbitError(orbit_error) => format!("we failed to calculate an orbit({})", honeyholt_brief!(orbit_error)),
    MassTooLowForMainSequence => "its mass is too low to be a main-sequence star".to_string(),
    MassTooHighForMainSequence => "its mass is too high to be a main-sequence star".to_string(),
    TooYoungToSupportLife => "it is too young to support life".to_string(),
    MassTooLowToSupportLife => "its mass is too low to support life".to_string(),
    MassTooHighToSupportLife => "its mass is too high to support life".to_string(),
  }
});
