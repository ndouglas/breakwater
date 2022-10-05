/// Constraints for creating a stellar neighborhood.
pub struct StellarNeighborhoodConstraints {
  /// The radius of the neighborhood, in light years.
  pub radius: Option<f64>,
  /// The density of the neighborhood, in stars per cubic light year.
  pub density: Option<f64>,
}
