/// Constraints for creating a star.
///
/// This is intended to ease creating stars with specific characteristics.
pub struct StarConstraints {
  /// The minimum mass of the star.
  pub minimum_stellar_mass: Option<f64>,
  /// The maximum mass of the star.
  pub maximum_stellar_mass: Option<f64>,
}
