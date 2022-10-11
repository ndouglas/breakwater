use crate::astronomy::stellar_neighbor::constraints::Constraints as StellarNeighborConstraints;
use crate::astronomy::stellar_neighborhood::constants::STELLAR_NEIGHBORHOOD_DENSITY;
use crate::astronomy::stellar_neighborhood::constants::STELLAR_NEIGHBORHOOD_RADIUS;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// The density of the neighborhood, in stars per cubic light year.
  pub density: Option<f64>,
  /// Any constraints placed on the various neighbors.
  pub neighbor_constraints: Option<StellarNeighborConstraints>,
}

impl Constraints {
  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let neighbor_constraints = Some(StellarNeighborConstraints::habitable());
    Self {
      neighbor_constraints,
      ..Constraints::default()
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(STELLAR_NEIGHBORHOOD_RADIUS);
    let density = Some(STELLAR_NEIGHBORHOOD_DENSITY);
    let neighbor_constraints = Some(StellarNeighborConstraints::default());
    Self {
      radius,
      density,
      neighbor_constraints,
    }
  }
}
