use crate::astronomy::planet::Planet;

pub mod constants;
use constants::*;
pub mod constraints;
pub mod error;
use error::Error;

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
}

impl Moon {
  #[named]
  pub fn from_mass(mass: f64, planet: &Planet, distance: f64) -> Result<Moon, Error> {
    trace_enter!();
    trace_var!(planet);
    trace_var!(distance);
    let result = Moon { mass };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
