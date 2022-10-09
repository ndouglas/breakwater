use rand::prelude::*;

use crate::astronomy::constants::MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::constants::MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MAXIMUM_HABITABLE_CLOSE_BINARY_STAR_SEPARATION;
use crate::astronomy::constants::MINIMUM_BINARY_STAR_SEPARATION;
use crate::astronomy::constants::MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::constants::MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::constants::MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::star_subsystem::constraints::Constraints as SubsystemConstraints;
use crate::astronomy::star_subsystem::Subsystem;

pub mod error;
use error::*;
pub mod orbit_type;
use orbit_type::*;

/// Details about the orbital information of a subsystem.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryConfiguration {
  /// The primary (larger mass) subsystem.
  pub primary: Subsystem,
  /// The secondary subsystem.
  pub secondary: Subsystem,
  /// The total mass, in Msol.
  pub mass: f64,
  /// The count of stars.
  pub star_count: u8,
  /// The total luminosity, in Lsol.
  pub luminosity: f64,
  /// Average separation of the binary components, in AU.
  pub average_separation: f64,
  /// Minimum separation of the components, in AU.
  pub minimum_separation: f64,
  /// Maximum separation of the components, in AU.
  pub maximum_separation: f64,
  /// Orbital eccentricity of the components.
  pub orbital_eccentricity: f64,
  /// Average distance from barycenter of the components.
  pub average_distances_from_barycenter: (f64, f64),
  /// Minumum distance from barycenter of the components.
  pub minimum_distances_from_barycenter: (f64, f64),
  /// Maxumum distance from barycenter of the components.
  pub maximum_distances_from_barycenter: (f64, f64),
  /// Area in which nothing can exist.
  pub forbidden_zone: (f64, f64),
  /// Area in which nothing _habitable_ can exist.
  pub danger_zone: (f64, f64),
  /// Habitable zone.
  pub habitable_zone: (f64, f64),
  /// Satellite bounds.
  pub satellite_zone: (f64, f64),
  /// The frost line.
  pub frost_line: f64,
  /// Whether this is amenable to a P-type or S-type orbit.
  pub orbit_type: OrbitType,
  /// Whether the habitable zone is contained within the forbidden zone.
  pub habitable_zone_is_forbidden: bool,
  /// Whether the habitable zone is contained within the danger zone.
  pub habitable_zone_is_dangerous: bool,
  /// The name of the primary star in the system.
  pub name: String,
}

impl BinaryConfiguration {
  /// Generate a random subsystem with the specified constraints.
  ///
  /// This may or may not be habitable, depending on the constraints.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &SubsystemConstraints,
  ) -> Result<BinaryConfiguration, Error> {
    trace_enter!();
    // Determine whether we're creating a close binary or a distant binary.
    let SubsystemConstraints {
      enable_close_binaries,
      enable_distant_binaries,
      force_close_binary,
      force_distant_binary,
      ..
    } = *constraints;
    let is_close_binary: bool = {
      if force_close_binary || !enable_distant_binaries {
        true
      } else if force_distant_binary || !enable_close_binaries {
        false
      } else {
        rng.gen()
      }
    };
    trace_var!(is_close_binary);
    // Prevent nesting more binaries within close binaries.
    let mut forward_constraints = constraints.clone();
    if is_close_binary {
      forward_constraints.enable_close_binaries = false;
    } else {
      forward_constraints.enable_close_binaries = true;
    }
    forward_constraints.enable_distant_binaries = false;
    let default_minimum_average_separation = {
      if is_close_binary {
        MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION
      } else {
        MINIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION
      }
    };
    trace_var!(default_minimum_average_separation);
    let default_maximum_average_separation = {
      if is_close_binary {
        MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION
      } else {
        MAXIMUM_DISTANT_BINARY_STAR_AVERAGE_SEPARATION
      }
    };
    trace_var!(default_maximum_average_separation);
    let minimum_separation_constraint = forward_constraints
      .minimum_separation
      .unwrap_or(default_minimum_average_separation);
    trace_var!(minimum_separation_constraint);
    let maximum_separation_constraint = forward_constraints
      .maximum_separation
      .unwrap_or(default_maximum_average_separation);
    trace_var!(maximum_separation_constraint);
    let minimum_orbital_eccentricity = forward_constraints
      .minimum_orbital_eccentricity
      .unwrap_or(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    trace_var!(minimum_orbital_eccentricity);
    let maximum_orbital_eccentricity = forward_constraints
      .maximum_orbital_eccentricity
      .unwrap_or(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    trace_var!(maximum_orbital_eccentricity);
    let average_separation = rng.gen_range(minimum_separation_constraint..maximum_separation_constraint);
    trace_var!(average_separation);
    let orbital_eccentricity = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
    trace_var!(orbital_eccentricity);
    let (primary, secondary) = {
      let sub_a = Subsystem::get_random_constrained(rng, &forward_constraints)?;
      let sub_b = Subsystem::get_random_constrained(rng, &forward_constraints)?;
      let sub_a_mass = sub_a.mass;
      let sub_b_mass = sub_b.mass;
      let first = if sub_a_mass > sub_b_mass {
        sub_a.clone()
      } else {
        sub_b.clone()
      };
      let second = if sub_a_mass > sub_b_mass {
        sub_b.clone()
      } else {
        sub_a.clone()
      };
      (first, second)
    };
    let primary_mass = primary.mass;
    trace_var!(primary_mass);
    let secondary_mass = secondary.mass;
    trace_var!(secondary_mass);
    let mass = primary_mass + secondary_mass;
    trace_var!(mass);
    let star_count = primary.star_count + secondary.star_count;
    trace_var!(star_count);
    let luminosity = primary.luminosity + secondary.luminosity;
    trace_var!(luminosity);
    let average_distances_from_barycenter = {
      let d1 = average_separation * (secondary_mass / mass);
      let d2 = average_separation * (primary_mass / mass);
      (d1, d2)
    };
    trace_var!(average_distances_from_barycenter);
    let minimum_distances_from_barycenter = {
      let d1 = average_distances_from_barycenter.0 * (1.0 - orbital_eccentricity);
      let d2 = average_distances_from_barycenter.1 * (1.0 - orbital_eccentricity);
      (d1, d2)
    };
    trace_var!(minimum_distances_from_barycenter);
    let minimum_separation = minimum_distances_from_barycenter.0 + minimum_distances_from_barycenter.1;
    trace_var!(minimum_separation);
    if minimum_separation < MINIMUM_BINARY_STAR_SEPARATION {
      return Err(Error::BinaryStarsTooCloseForComfort);
    }
    let maximum_distances_from_barycenter = {
      let d1 = average_distances_from_barycenter.0 * (1.0 + orbital_eccentricity);
      let d2 = average_distances_from_barycenter.1 * (1.0 + orbital_eccentricity);
      (d1, d2)
    };
    trace_var!(maximum_distances_from_barycenter);
    let maximum_separation = maximum_distances_from_barycenter.0 + maximum_distances_from_barycenter.1;
    trace_var!(maximum_separation);
    let forbidden_zone = (minimum_separation / 3.0, maximum_separation * 3.0);
    trace_var!(forbidden_zone);
    let danger_zone = (0.0, maximum_separation * 4.0);
    trace_var!(danger_zone);
    let habitable_zone = ((luminosity / 1.1).sqrt(), (luminosity / 0.53).sqrt());
    trace_var!(habitable_zone);
    let satellite_zone = (0.1 * mass, 40.0 * mass);
    trace_var!(satellite_zone);
    let frost_line = 4.85 * luminosity.sqrt();
    trace_var!(frost_line);
    let orbit_type = {
      use OrbitType::*;
      if maximum_separation <= MAXIMUM_HABITABLE_CLOSE_BINARY_STAR_SEPARATION {
        PType
      } else if primary.is_habitable() && secondary.is_habitable() {
        STypeBoth
      } else if primary.is_habitable() {
        STypePrimary
      } else if secondary.is_habitable() {
        STypeSecondary
      } else {
        None
      }
    };
    let habitable_zone_is_forbidden = habitable_zone.1 <= forbidden_zone.1;
    trace_var!(habitable_zone_is_forbidden);
    let habitable_zone_is_dangerous = habitable_zone.1 <= danger_zone.1;
    trace_var!(habitable_zone_is_dangerous);
    let name = primary.name.clone();
    trace_var!(name);
    let result = BinaryConfiguration {
      primary,
      secondary,
      mass,
      star_count,
      luminosity,
      average_separation,
      minimum_separation,
      maximum_separation,
      orbital_eccentricity,
      average_distances_from_barycenter,
      minimum_distances_from_barycenter,
      maximum_distances_from_barycenter,
      forbidden_zone,
      danger_zone,
      habitable_zone,
      satellite_zone,
      frost_line,
      orbit_type,
      habitable_zone_is_forbidden,
      habitable_zone_is_dangerous,
      name,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Indicate whether this subsystem is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    use OrbitType::*;
    trace_enter!();
    let result = match self.orbit_type {
      None => return Err(Error::NoHabitableZoneFound),
      PType => {
        if self.habitable_zone_is_forbidden {
          return Err(Error::HabitableZoneContainedWithinForbiddenZone);
        }
        if self.habitable_zone_is_dangerous {
          return Err(Error::HabitableZoneContainedWithinDangerZone);
        }
        Ok(())
      },
      STypePrimary | STypeSecondary | STypeBoth => Ok(()),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}
