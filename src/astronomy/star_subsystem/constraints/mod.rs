use std::default::Default;

use crate::astronomy::constants::MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::constants::MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::constants::MAXIMUM_STAR_SUBSYSTEM_RECURSION;
use crate::astronomy::constants::MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::constants::MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MINIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::constants::PROBABILITY_OF_BINARY_STARS;
use crate::astronomy::star::constraints::Constraints as StarConstraints;

/// Constraints for creating a main-sequence star subsystem.
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
  /// Enforce habitability (at the top level).
  pub enforce_habitability: bool,
  /// Enable creating close binary systems.
  pub enable_close_binaries: bool,
  /// Enable creating distant binary systems.
  pub enable_distant_binaries: bool,
  /// Force creatign a solitary system.
  pub force_solitary: bool,
  /// Force creating a close binary system.
  pub force_close_binary: bool,
  /// Force creating a distant binary system.
  pub force_distant_binary: bool,
}

impl Constraints {
  /// Generate a habitable star subsystem, force solitary.
  pub fn habitable_solitary() -> Self {
    let binary_probability = None;
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    let enforce_habitability = true;
    let enable_close_binaries = false;
    let enable_distant_binaries = false;
    let force_solitary = true;
    let force_close_binary = false;
    let force_distant_binary = false;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
      enforce_habitability,
      enable_close_binaries,
      enable_distant_binaries,
      force_solitary,
      force_close_binary,
      force_distant_binary,
    }
  }

  /// Generate a habitable star subsystem.
  pub fn habitable_close_binary() -> Self {
    let binary_probability = None;
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    let enforce_habitability = true;
    let enable_close_binaries = true;
    let enable_distant_binaries = false;
    let force_solitary = false;
    let force_close_binary = true;
    let force_distant_binary = false;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
      enforce_habitability,
      enable_close_binaries,
      enable_distant_binaries,
      force_solitary,
      force_close_binary,
      force_distant_binary,
    }
  }

  /// Generate a habitable star subsystem.
  pub fn habitable_distant_binary() -> Self {
    let binary_probability = None;
    let minimum_separation = Some(MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::habitable());
    let maximum_depth = MAXIMUM_STAR_SUBSYSTEM_RECURSION;
    let enforce_habitability = true;
    let enable_close_binaries = false;
    let enable_distant_binaries = true;
    let force_solitary = false;
    let force_close_binary = false;
    let force_distant_binary = true;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
      enforce_habitability,
      enable_close_binaries,
      enable_distant_binaries,
      force_solitary,
      force_close_binary,
      force_distant_binary,
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let binary_probability = Some(PROBABILITY_OF_BINARY_STARS);
    let minimum_separation = Some(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    let maximum_separation = Some(MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION);
    let minimum_orbital_eccentricity = Some(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_DISTANT_BINARY_STAR_ORBITAL_ECCENTRICITY);
    let star_constraints = Some(StarConstraints::default());
    let maximum_depth = 4;
    let enforce_habitability = false;
    let enable_close_binaries = true;
    let enable_distant_binaries = true;
    let force_solitary = false;
    let force_close_binary = false;
    let force_distant_binary = false;
    Self {
      binary_probability,
      minimum_separation,
      maximum_separation,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      star_constraints,
      maximum_depth,
      enforce_habitability,
      enable_close_binaries,
      enable_distant_binaries,
      force_solitary,
      force_close_binary,
      force_distant_binary,
    }
  }
}
