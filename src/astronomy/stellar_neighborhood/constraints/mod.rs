use crate::astronomy::StellarNeighborConstraints;

/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StellarNeighborhoodConstraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// The density of the neighborhood, in stars per cubic light year.
  pub density: Option<f64>,
  /// Any constraints placed on the various neighbors.
  pub neighbor_constraints: Option<StellarNeighborConstraints>,
}
