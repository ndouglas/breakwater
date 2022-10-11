use std::default::Default;

use crate::astronomy::star_system::constraints::Constraints as SystemConstraints;
use crate::astronomy::stellar_neighborhood::constants::STELLAR_NEIGHBORHOOD_RADIUS;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// Star system constraints.
  pub system_constraints: Option<SystemConstraints>,
}

impl Constraints {
  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let system_constraints = Some(SystemConstraints::habitable());
    Self {
      system_constraints,
      ..Constraints::default()
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let radius = Some(STELLAR_NEIGHBORHOOD_RADIUS);
    let system_constraints = Some(SystemConstraints::default());
    Self {
      radius,
      system_constraints,
    }
  }
}
