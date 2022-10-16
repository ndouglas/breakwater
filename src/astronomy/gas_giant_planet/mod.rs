pub mod constants;
pub mod constraints;
pub mod error;

/// The `GasGiantPlanet` type.
#[derive(Clone, Debug, PartialEq)]
pub struct GasGiantPlanet {
  /// Mass, in Mjupiter.
  pub mass: f64,
}
