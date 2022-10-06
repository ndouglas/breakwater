use rand::prelude::*;

use crate::astronomy::AstronomicalError;
use crate::astronomy::star_system::subsystem::Subsystem;
use crate::astronomy::star_system::subsystem::constraints::Constraints as SubsystemConstraints;
use crate::astronomy::MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::MINIMUM_BINARY_STAR_SEPARATION;
use crate::astronomy::MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION;
use crate::astronomy::MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY;
use crate::astronomy::MAXIMUM_HABITABLE_CLOSE_BINARY_STAR_SEPARATION;

pub mod orbit_type;
use orbit_type::*;

/// Details about the orbital information of a subsystem.
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryConfiguration {
  /// Average separation of the binary components, in AU.
  pub average_separation: f64,
  /// Minimum separation of the components, in AU.
  pub minimum_separation: f64,
  /// Maximum separation of the components, in AU.
  pub maximum_separation: f64,
  /// Orbital eccentricities of the components.
  pub orbital_eccentricities: (f64, f64),
  /// Average distance from barycenter of the components.
  pub average_distances_from_barycenter: (f64, f64),
  /// Minumum distance from barycenter of the components.
  pub minimum_distances_from_barycenter: (f64, f64),
  /// Maxumum distance from barycenter of the components.
  pub maximum_distances_from_barycenter: (f64, f64),
  /// Areas in which nothing can exist.
  pub forbidden_zone: (f64, f64),
  /// Whether this is amenable to a P-type or S-type orbit.
  pub orbit_type: OrbitType,
}

impl BinaryConfiguration {
  /// Generate a random subsystem with the specified constraints.
  ///
  /// This may or may not be habitable, depending on the constraints.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    sub1: &Subsystem,
    sub2: &Subsystem,
    constraints: &SubsystemConstraints,
  ) -> Result<BinaryConfiguration, AstronomicalError> {
    trace_enter!();
    let minimum_separation_constraint = constraints
      .minimum_separation
      .unwrap_or(MINIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    trace_var!(minimum_separation_constraint);
    let maximum_separation_constraint = constraints
      .maximum_separation
      .unwrap_or(MAXIMUM_CLOSE_BINARY_STAR_AVERAGE_SEPARATION);
    trace_var!(maximum_separation_constraint);
    let minimum_orbital_eccentricity = constraints
      .minimum_orbital_eccentricity
      .unwrap_or(MINIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    trace_var!(minimum_orbital_eccentricity);
    let maximum_orbital_eccentricity = constraints
      .maximum_orbital_eccentricity
      .unwrap_or(MAXIMUM_CLOSE_BINARY_STAR_ORBITAL_ECCENTRICITY);
    trace_var!(maximum_orbital_eccentricity);
    let average_separation = rng.gen_range(minimum_separation_constraint..maximum_separation_constraint);
    trace_var!(average_separation);
    let orbital_eccentricities = {
      let o1 = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
      let o2 = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
      (o1, o2)
    };
    trace_var!(orbital_eccentricities);
    let sub1_mass = sub1.get_mass();
    trace_var!(sub1_mass);
    let sub2_mass = sub2.get_mass();
    trace_var!(sub2_mass);
    let total_mass = sub1_mass + sub2_mass;
    trace_var!(total_mass);
    let average_distances_from_barycenter = {
      let d1 = average_separation * (sub2_mass / total_mass);
      let d2 = average_separation * (sub1_mass / total_mass);
      (d1, d2)
    };
    trace_var!(average_distances_from_barycenter);
    let minimum_distances_from_barycenter = {
      let d1 = average_distances_from_barycenter.0 * (1.0 - orbital_eccentricities.0);
      let d2 = average_distances_from_barycenter.1 * (1.0 - orbital_eccentricities.1);
      (d1, d2)
    };
    trace_var!(minimum_distances_from_barycenter);
    let minimum_separation = minimum_distances_from_barycenter.0 + minimum_distances_from_barycenter.1;
    trace_var!(minimum_separation);
    if minimum_separation < MINIMUM_BINARY_STAR_SEPARATION {
      return Err(AstronomicalError::BinaryStarsTooCloseForComfort);
    }
    let maximum_distances_from_barycenter = {
      let d1 = average_distances_from_barycenter.0 * (1.0 + orbital_eccentricities.0);
      let d2 = average_distances_from_barycenter.1 * (1.0 + orbital_eccentricities.1);
      (d1, d2)
    };
    trace_var!(maximum_distances_from_barycenter);
    let maximum_separation = maximum_distances_from_barycenter.0 + maximum_distances_from_barycenter.1;
    trace_var!(maximum_separation);
    let forbidden_zone = (minimum_separation / 3.0, maximum_separation * 3.0);
    trace_var!(forbidden_zone);
    let orbit_type = {
      use OrbitType::*;
      if maximum_separation <= MAXIMUM_HABITABLE_CLOSE_BINARY_STAR_SEPARATION {
        PType
      } else if sub1.is_habitable() && sub2.is_habitable() {
        STypeBoth
      } else if sub1.is_habitable() {
        STypePrimary
      } else if sub2.is_habitable() {
        STypeSecondary
      } else {
        None
      }
    };
    let result = BinaryConfiguration {
      average_separation,
      minimum_separation,
      maximum_separation,
      orbital_eccentricities,
      average_distances_from_barycenter,
      minimum_distances_from_barycenter,
      maximum_distances_from_barycenter,
      forbidden_zone,
      orbit_type,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

}
