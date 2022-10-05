/// Constraints for creating a stellar neighborhood.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StellarNeighborConstraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
}
