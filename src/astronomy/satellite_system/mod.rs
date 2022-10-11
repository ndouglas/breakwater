use crate::astronomy::moons::Moons;
use crate::astronomy::planet::Planet;

pub mod constants;
pub mod constraints;
pub mod error;

/// A `SatelliteSystem` is a collection of a `Planet` and `Moons`.
#[derive(Clone, Debug, PartialEq)]
pub struct SatelliteSystem {
  /// The planet.
  pub planet: Planet,
  /// The moons.
  pub moons: Moons,
}
