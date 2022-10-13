use rand::distributions::{Distribution, Standard};
use rand::prelude::*;

pub mod constants;
use constants::*;
pub mod constraints;
pub mod error;

/// A `Moon`, mercifully, is a fairly simple concept.
///
/// It's possible that at some point, we might make moons habitable.
///
/// For instance, a habitable moon of a hot jupiter gas giant.
///
/// But for now, we're just staying with terrestrial planets, and we'll assume
/// that moons are just celestial features.
#[derive(Clone, Debug, PartialEq)]
pub struct Moon {
  /// The mass of this moon.
  pub mass: f64,
  /// The albedo (geometric) of this moon.
  pub albedo: f64,
}

impl Distribution<Moon> for Standard {
  #[named]
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Moon {
    let mass = rng.gen_range(MINIMUM_MASS..MAXIMUM_MASS);
    let albedo = rng.gen_range(MINIMUM_ALBEDO..MAXIMUM_ALBEDO);
    Moon { mass, albedo }
  }
}
