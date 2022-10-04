/// The `StellarNeighborhood` type.
///
/// This is mostly a container for star systems.
pub struct StellarNeighborhood {
  /// The radius of this neighborhood, measured in light years.
  pub radius: f64,
  /// The stellar density of this neighborhood, measured in stars per cubic
  /// light year.  This is not terribly useful once the neighborhood has
  /// been generated, but we keep it around for posterity.
  pub density: f64,
}
