use std::default::Default;

/// Constraints for creating a set of orbits about a star.
///
/// This is intended to ease creating planetary systems.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Enforce a maximum number of planets.
  pub maximum_orbits: Option<usize>,
  /// Enforce a minimum distance from the host star, in AU.
  pub minimum_distance: Option<f64>,
  /// Enforce a maximum distance from the host star, in AU.
  pub maximum_distance: Option<f64>,
  /// Include a habitable planet?
  pub include_habitable_planet: Option<bool>,
  /// Include a primary gas giant?
  pub include_primary_gas_giant: Option<bool>,
}

impl Constraints {
  /// Generate an orbit in the habitable zone around a star.
  #[named]
  pub fn habitable() -> Self {
    let include_habitable_planet = Some(true);
    let include_primary_gas_giant = Some(true);
    Self {
      include_habitable_planet,
      include_primary_gas_giant,
      ..Constraints::default()
    }
  }

}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  #[named]
  fn default() -> Self {
    let maximum_orbits = None;
    let minimum_distance = None;
    let maximum_distance = None;
    let include_habitable_planet = None;
    let include_primary_gas_giant = None;
    Self {
      maximum_orbits,
      minimum_distance,
      maximum_distance,
      include_habitable_planet,
      include_primary_gas_giant,
    }
  }
}
