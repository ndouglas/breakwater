pub mod constants;
pub mod constraints;
pub mod error;

/// The `Planet` class.  This will get complicated.
#[derive(Clone, Debug, PartialEq)]
pub struct Planet {
  /// The mass of this planet.
  pub mass: f64,
  /// The albedo (geometric) of this planet.
  pub albedo: f64,
}
