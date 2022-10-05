/// Constraints for creating a star.
///
/// This is intended to ease creating stars with specific characteristics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StarConstraints {
  /// The minimum mass of the star, in Msol.
  pub minimum_mass: Option<f64>,
  /// The maximum mass of the star, in Msol.
  pub maximum_mass: Option<f64>,
  /// The minimum age of the star, in Gyr.
  pub minimum_age: Option<f64>,
  /// The maximum age of the star, in Gyr.
  pub maximum_age: Option<f64>,
}
