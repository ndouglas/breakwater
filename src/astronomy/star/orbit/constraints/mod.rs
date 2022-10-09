use std::default::Default;

use super::super::math::orbit::{get_approximate_innermost_orbit, get_approximate_outermost_orbit};
use super::error::Error;

/// Constraints for creating an orbit about a star.
///
/// This is intended to ease creating orbits with specific characteristics.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Force this to be within or not within the habitable zone.
  pub enforce_habitability: bool,
  /// Force this to be within the frost line.
  pub enforce_inside_frost_line: bool,
  /// Force this to be outside the frost line.
  pub enforce_outside_frost_line: bool,
  /// Enforce a minimum distance from the host star, in AU.
  pub minimum_distance: Option<f64>,
  /// Enforce a maximum distance from the host star, in AU.
  pub maximum_distance: Option<f64>,
  /// A "primary gas giant" preset.
  pub make_primary_gas_giant: bool,
}

impl Constraints {
  /// Generate an orbit in the habitable zone around a star.
  #[named]
  pub fn habitable() -> Self {
    let enforce_habitability = true;
    Self {
      enforce_habitability,
      ..Constraints::default()
    }
  }

  /// Generate a primary gas giant, who lurks outside the frost line.
  #[named]
  pub fn primary_gas_giant() -> Self {
    let make_primary_gas_giant = true;
    Self {
      make_primary_gas_giant,
      ..Constraints::default()
    }
  }

  /// Determine an inner bound, given our constraints, a habitable zone, and
  /// the frost line.
  #[named]
  pub fn get_final_minimum_distance(
    &self,
    mass: f64,
    habitable_zone: (f64, f64),
    frost_line: f64,
  ) -> Result<f64, Error> {
    trace_enter!();
    trace_var!(mass);
    trace_var!(habitable_zone);
    trace_var!(frost_line);
    let minimum_distance = self.minimum_distance.unwrap_or(get_approximate_innermost_orbit(mass));
    trace_var!(minimum_distance);
    let maximum_distance = self.maximum_distance.unwrap_or(get_approximate_outermost_orbit(mass));
    trace_var!(maximum_distance);
    let result = {
      if self.make_primary_gas_giant {
        if minimum_distance >= frost_line {
          minimum_distance
        } else {
          frost_line * 1.2
        }
      } else if self.enforce_habitability {
        if minimum_distance >= habitable_zone.0 && minimum_distance <= habitable_zone.1 {
          minimum_distance
        } else {
          habitable_zone.0
        }
      } else if self.enforce_outside_frost_line {
        frost_line
      } else {
        minimum_distance
      }
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Determine an outer bound, given our constraints, a habitable zone, and
  /// the frost line.
  #[named]
  pub fn get_final_maximum_distance(
    &self,
    mass: f64,
    habitable_zone: (f64, f64),
    frost_line: f64,
  ) -> Result<f64, Error> {
    trace_enter!();
    trace_var!(mass);
    trace_var!(habitable_zone);
    trace_var!(frost_line);
    let minimum_distance = self.minimum_distance.unwrap_or(get_approximate_innermost_orbit(mass));
    trace_var!(minimum_distance);
    let maximum_distance = self.maximum_distance.unwrap_or(get_approximate_outermost_orbit(mass));
    trace_var!(maximum_distance);
    let result = {
      if self.make_primary_gas_giant {
        frost_line * 1.4
      } else if self.enforce_habitability {
        if maximum_distance >= habitable_zone.0 && maximum_distance <= habitable_zone.1 {
          maximum_distance
        } else {
          habitable_zone.1
        }
      } else if self.enforce_inside_frost_line {
        frost_line
      } else {
        maximum_distance
      }
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  #[named]
  fn default() -> Self {
    let enforce_habitability = false;
    let enforce_inside_frost_line = false;
    let enforce_outside_frost_line = false;
    let minimum_distance = None;
    let maximum_distance = None;
    let make_primary_gas_giant = false;
    Self {
      enforce_habitability,
      enforce_inside_frost_line,
      enforce_outside_frost_line,
      minimum_distance,
      maximum_distance,
      make_primary_gas_giant,
    }
  }
}
