use crate::astronomy::gas_giant_planet::error::Error as GasGiantPlanetError;
use crate::astronomy::terrestrial_planet::error::Error as TerrestrialPlanetError;

/// Planet errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// GasGiantPlanet error.
  GasGiantPlanetError(GasGiantPlanetError),
  /// TerrestrialPlanet error.
  TerrestrialPlanetError(TerrestrialPlanetError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    GasGiantPlanetError(gas_giant_planet_error) => format!(
      "an error occurred in the gas giant planet ({})",
      honeyholt_brief!(gas_giant_planet_error)
    ),
    TerrestrialPlanetError(terrestrial_planet_error) => format!(
      "an error occurred in the terrestrial planet ({})",
      honeyholt_brief!(terrestrial_planet_error)
    ),
  }
});

impl From<GasGiantPlanetError> for Error {
  #[named]
  fn from(error: GasGiantPlanetError) -> Self {
    Error::GasGiantPlanetError(error)
  }
}

impl From<TerrestrialPlanetError> for Error {
  #[named]
  fn from(error: TerrestrialPlanetError) -> Self {
    Error::TerrestrialPlanetError(error)
  }
}
