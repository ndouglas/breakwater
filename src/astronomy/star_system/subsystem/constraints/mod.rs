use std::default::Default;

use crate::astronomy::StarConstraints;
use crate::astronomy::MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MINIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::PROBABILITY_OF_BINARY_STARS;
use crate::astronomy::MAXIMUM_STAR_SUBSYSTEM_RECURSION;

/// Constraints for creating a star subsystem.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The probability that a star is binary rather than single.
  pub binary_probability: Option<f64>,
  /// The minimum separation between binary stars, in AU.
  pub minimum_separation: Option<f64>,
  /// The maximum separation between binary stars, in AU.
  pub maximum_separation: Option<f64>,
  /// The minimum orbital eccentricity of binary stars.
  pub minimum_orbital_eccentricity: Option<f64>,
  /// The maximum orbital eccentricity of binary stars.
  pub maximum_orbital_eccentricity: Option<f64>,
  /// Star creation constraints.
  pub star_constraints: Option<StarConstraints>,
  /// Maximum depth.
  pub maximum_depth: u8,
}

impl Constraints {
  /// Generate a main-sequence star subsystem.
  pub fn main_sequence() -> Self {
    let binary_probability = Some(PROBABILITY_OF_BINARY_STARS);
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::main_sequence());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }

  /// Generate a habitable star subsystem, force solitary.
  pub fn habitable_solitary() -> Self {
    let binary_probability = Some(0.00);
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }

  /// Generate a habitable star subsystem.
  pub fn habitable_solitary_or_p_type_binary() -> Self {
    let binary_probability = Some(PROBABILITY_OF_BINARY_STARS);
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }

  /// Generate a habitable star subsystem.
  pub fn habitable_p_type_binary() -> Self {
    let binary_probability = Some(100.0);
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }

  /// Generate a habitable star subsystem.
  pub fn habitable_solitary_or_s_type_binary() -> Self {
    let binary_probability = Some(PROBABILITY_OF_BINARY_STARS);
    let minimum_separation = Some(MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }

  /// Generate a habitable star subsystem.
  pub fn habitable_s_type_binary() -> Self {
    let binary_probability = Some(100.0);
    let minimum_separation = Some(MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }

}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let binary_probability = Some(PROBABILITY_OF_BINARY_STARS);
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::default());
    let maximum_depth = 4;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
    }
  }
}
