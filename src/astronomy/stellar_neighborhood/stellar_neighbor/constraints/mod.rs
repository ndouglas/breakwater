use std::default::Default;

use crate::astronomy::star_system::constraints::Constraints as StarSystemConstraints;
use crate::astronomy::RADIUS_OF_STELLAR_NEIGHBORHOOD;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StellarNeighborConstraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// Star system constraints.
  pub star_system_constraints: Option<StarSystemConstraints>,
}

impl StellarNeighborConstraints {
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let star_system_constraints = Some(StarSystemConstraints::main_sequence());
    Self {
      radius,
      star_system_constraints,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let star_system_constraints = Some(StarSystemConstraints::habitable());
    Self {
      radius,
      star_system_constraints,
    }
  }
}

impl Default for StellarNeighborConstraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let star_system_constraints = Some(StarSystemConstraints::default());
    Self {
      radius,
      star_system_constraints,
    }
  }
}
