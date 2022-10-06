use crate::astronomy::stellar_neighborhood::stellar_neighbor::constraints::Constraints as StellarNeighborConstraints;
use crate::astronomy::constants::DENSITY_OF_STELLAR_NEIGHBORHOOD;
use crate::astronomy::constants::RADIUS_OF_STELLAR_NEIGHBORHOOD;

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
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let density = Some(DENSITY_OF_STELLAR_NEIGHBORHOOD);
    let neighbor_constraints = Some(StellarNeighborConstraints::main_sequence());
    Self {
      radius,
      density,
      neighbor_constraints,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let density = Some(DENSITY_OF_STELLAR_NEIGHBORHOOD);
    let neighbor_constraints = Some(StellarNeighborConstraints::habitable());
    Self {
      radius,
      density,
      neighbor_constraints,
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let density = Some(DENSITY_OF_STELLAR_NEIGHBORHOOD);
    let neighbor_constraints = Some(StellarNeighborConstraints::default());
    Self {
      radius,
      density,
      neighbor_constraints,
    }
  }
}
