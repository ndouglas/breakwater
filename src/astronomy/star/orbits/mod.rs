use rand::prelude::*;

pub mod constraints;
use constraints::*;
pub mod error;
use error::*;

use crate::astronomy::star::math::orbit::{get_approximate_innermost_orbit, get_approximate_outermost_orbit};

use super::orbit::Orbit;
use super::orbit::constraints::Constraints as OrbitConstraints;

/// A set of potential (or actual) orbits for a star.
///
/// This exists mostly to encapsulate certain operations with creating orbits
/// and managing their interactions, etc.
#[derive(Clone, Debug, PartialEq)]
pub struct Orbits {
  /// Actual orbit objects.
  pub orbits: Vec<Orbit>,
}

impl Orbits {
  /// Generate from a constraint.
  #[named]
  pub fn from_constraints<R: Rng + ?Sized>(rng: &mut R, mass: f64, constraints: &Constraints) -> Result<Self, Error> {
    trace_enter!();
    trace_var!(mass);
    trace_var!(constraints);
    let mut orbits = Vec::new();
    let mut exclusions: Vec<f64> = vec![];
    if constraints.include_primary_gas_giant.unwrap_or(false) {
      let orbit = Orbit::from_constraints(rng, mass, &OrbitConstraints::primary_gas_giant())?;
      trace_var!(orbit);
      exclusions.push(orbit.distance);
      assert!(orbit.is_outside_frost_line);
      orbits.push(orbit);
    }
    if constraints.include_habitable_planet.unwrap_or(false) {
      let orbit = Orbit::from_constraints(rng, mass, &OrbitConstraints::habitable())?;
      trace_var!(orbit);
      exclusions.push(orbit.distance);
      assert!(orbit.is_in_habitable_zone);
      orbits.push(orbit);  
    }
    let approximate_satellite_inner_bound = get_approximate_innermost_orbit(mass);
    trace_var!(approximate_satellite_inner_bound);
    let approximate_satellite_outer_bound = get_approximate_outermost_orbit(mass);
    trace_var!(approximate_satellite_outer_bound);
    let minimum = 40.0 * approximate_satellite_inner_bound;
    trace_var!(minimum);
    let distance_limit = approximate_satellite_outer_bound;
    trace_var!(distance_limit);
    let mut growth_factor = 0.3;
    trace_var!(growth_factor);
    let mut orbital_distance = minimum;
    let mut index = 0;
    loop {
      let min_unwrapped = 0.95 * orbital_distance;
      let max_unwrapped = 1.05 * orbital_distance;
      if !exclusions
        .iter()
        .any(|&exclusion| exclusion > min_unwrapped && exclusion < max_unwrapped)
      {
        let minimum_distance = Some(min_unwrapped);
        let maximum_distance = Some(max_unwrapped);
        let orbit = Orbit::from_constraints(
          rng,
          mass,
          &OrbitConstraints {
            minimum_distance,
            maximum_distance,
            ..OrbitConstraints::default()
          },
        )?;
        trace_var!(orbit);
        orbits.push(orbit);
      }
      orbital_distance = minimum + growth_factor * (2.0_f64).powf(index as f64);
      index += 1;
      if orbital_distance > distance_limit {
        break;
      }
    }
    trace_var!(orbits);
    orbits.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());      
    trace_var!(orbits);
    let result = Orbits {
      orbits,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_from_constraints() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let orbits = Orbits::from_constraints(&mut rng, 1.0, &Constraints::default())?;
    trace_var!(orbits);
    print_var!(orbits);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let orbits = Orbits::from_constraints(&mut rng, 1.0, &Constraints::habitable())?;
    trace_var!(orbits);
    print_var!(orbits);
    assert!(orbits.orbits.iter().any(|&orbit| orbit.is_in_habitable_zone));
    trace_exit!();
    Ok(())
  }

}
