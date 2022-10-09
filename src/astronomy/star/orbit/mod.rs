use rand::prelude::*;

use super::math::luminosity::ms_star_mass_to_luminosity;
use super::math::orbit::{get_approximate_innermost_orbit, get_approximate_outermost_orbit};
use super::math::frost_line::ms_star_luminosity_to_frost_line;
use super::math::habitable_zone::ms_star_luminosity_to_habitable_zone;

pub mod constraints;
use constraints::Constraints;
pub mod error;
use error::Error;

/// Collects information about a particular orbit around a host star.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Orbit {
  /// The distance of the orbit.
  pub distance: f64,
  /// The eccentricity of the orbit.
  pub eccentricity: f64,
  /// Whether this orbit is habitable relative to the host star.
  pub is_in_habitable_zone: bool,
  /// Whether this orbit is outside the frost line of the host star.
  pub is_outside_frost_line: bool,
}

impl Orbit {
  /// Generate from a constraint.
  #[named]
  pub fn from_constraint<R: Rng + ?Sized>(rng: &mut R, mass: f64, constraints: &Constraints) -> Result<Self, Error> {
    trace_enter!();
    let minimum_distance = constraints.minimum_distance.unwrap_or(get_approximate_innermost_orbit(mass));
    trace_var!(minimum_distance);
    let maximum_distance = constraints.maximum_distance.unwrap_or(get_approximate_outermost_orbit(mass));
    trace_var!(maximum_distance);
    let enforce_habitability = constraints.enforce_habitability;
    trace_var!(enforce_habitability);
    let enforce_inside_frost_line = constraints.enforce_inside_frost_line;
    trace_var!(enforce_inside_frost_line);
    let enforce_outside_frost_line = constraints.enforce_outside_frost_line;
    trace_var!(enforce_outside_frost_line);
    let make_primary_gas_giant = constraints.make_primary_gas_giant;
    trace_var!(make_primary_gas_giant);
    let luminosity = ms_star_mass_to_luminosity(mass).unwrap();
    trace_var!(luminosity);
    let habitable_zone = ms_star_luminosity_to_habitable_zone(luminosity);
    trace_var!(habitable_zone);
    let frost_line = ms_star_luminosity_to_frost_line(luminosity);
    trace_var!(frost_line);
    let inner_bound = constraints.get_final_minimum_distance(mass, habitable_zone, frost_line)?;
    trace_var!(inner_bound);
    let outer_bound = constraints.get_final_maximum_distance(mass, habitable_zone, frost_line)?;
    trace_var!(outer_bound);
    let distance = rng.gen_range(inner_bound..outer_bound);
    trace_var!(distance);
    let is_in_habitable_zone = distance > habitable_zone.0 && distance < habitable_zone.1;
    trace_var!(is_in_habitable_zone);
    let is_outside_frost_line = distance > frost_line;
    trace_var!(is_outside_frost_line);
    let eccentricity = rng.gen_range(0.02..0.06);
    trace_var!(eccentricity);
    let result = Orbit {
      distance,
      eccentricity,
      is_in_habitable_zone,
      is_outside_frost_line,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
