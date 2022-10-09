use rand::prelude::*;

pub mod constraints;
use constraints::*;

/// A set of potential (or actual) orbits for a star.
///
/// This exists mostly to encapsulate certain operations with creating orbits
/// and managing their interactions, etc.
pub struct Orbits {
  /// Actual orbit objects.
  pub orbits: Vec<Orbit>,
}

impl Orbits {
  /// Generate from a constraint.
  #[named]
  pub fn from_constraint<R: Rng + ?Sized>(rng: &mut R, mass: f64, constraints: &Constraints) -> Self {
    trace_enter!();
    trace_var!(mass);
    trace_var!(constraints);
    let mut orbits = Vec::new();
    
    let result = Orbits {
      orbits,
    }
    trace_var!(result);
    trace_exit!();
    result
  }

}