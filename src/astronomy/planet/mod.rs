use crate::astronomy::terrestrial_planet::TerrestrialPlanet;

pub mod constants;
pub mod constraints;
pub mod error;
pub mod math;

/// The `Planet` class.  This will get complicated.
#[derive(Clone, Debug, PartialEq)]
pub enum Planet {
  /// Gas Giant Planet.
  /// Terrestrial Planet.
  TerrestrialPlanet(TerrestrialPlanet),
}

impl Planet {}
