use crate::astronomy::planet::Planet;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;
pub mod math;
pub mod rotation_direction;
use rotation_direction::RotationDirection;

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
  /// The mass of this moon, in Mmoon.
  pub mass: f64,
  /// The density of this moon, in Dmoon.
  pub density: f64,
  /// The radius of this moon, in Rmoon.
  pub radius: f64,
  /// The gravity of this moon, in Gearth (not Gmoon).
  pub gravity: f64,
  /// The escape velocity of this moon, in Vearth.
  pub escape_velocity: f64,
  /// The Bond albedo of this moon.
  pub bond_albedo: f64,
  /// Semi-major axis, in KM.
  pub semi_major_axis: f64,
  /// Orbital eccentricity.
  pub orbital_eccentricity: f64,
  /// Periapsis.
  pub periapsis: f64,
  /// Apoapsis.
  pub apoapsis: f64,
  /// Orbital inclination.
  pub orbital_inclination: f64,
  /// Rotation direction.
  pub rotation_direction: RotationDirection,
  /// Sidereal orbital period.
  pub sidereal_orbital_period: f64,
  /// Normal orbital period.
  pub orbital_period: f64,
  /// Rotational period.
  pub rotation_period: f64,
}

impl Moon {
  #[named]
  pub fn from_mass(mass: f64, planet: &Planet, distance: f64) -> Result<Moon, Error> {
    trace_enter!();
    trace_var!(planet);
    trace_var!(distance);
    let density = 3.34;
    trace_var!(density);
    let radius = (mass / (density / 3.34)).powf(1.0 / 3.0);
    trace_var!(radius);
    // This gives gravity in Earth equivalents, since other units are relative
    // to the Moon, and Gmoon is 0.1654 * Gearth.
    let gravity = (mass / radius.powf(2.0)) * 0.1654;
    trace_var!(gravity);
    // This is in Vearth, not Vmoon, as a result of the constant.
    let escape_velocity = (mass / radius).sqrt() * 2.380;
    trace_var!(escape_velocity);
    // Peg this to the albedo of the Moon for the time being.
    let bond_albedo = 0.136;
    trace_var!(bond_albedo);
    let semi_major_axis = distance;
    trace_var!(semi_major_axis);
    // Pegged for the time being.
    let orbital_eccentricity = 0.05;
    trace_var!(orbital_eccentricity);
    let periapsis = (1.0 - orbital_eccentricity) * semi_major_axis;
    trace_var!(periapsis);
    let apoapsis = (1.0 + orbital_eccentricity) * semi_major_axis;
    trace_var!(apoapsis);
    // Pegged.
    let orbital_inclination = 5.15;
    trace_var!(orbital_inclination);
    let rotation_direction = RotationDirection::Prograde;
    trace_var!(rotation_direction);
    let sidereal_orbital_period =
      0.0588 * (((semi_major_axis / 12742.0) * 2.0).powf(3.0) / (planet.get_mass() + (mass * 0.0123))).sqrt();
    let orbital_period = planet.get_orbital_period() / (planet.get_orbital_period() / sidereal_orbital_period - 1.0);
    // This can't be calculated yet.
    let rotation_period = 3.0;
    trace_var!(rotation_period);
    let result = Moon {
      mass,
      density,
      radius,
      gravity,
      escape_velocity,
      bond_albedo,
      semi_major_axis,
      orbital_eccentricity,
      periapsis,
      apoapsis,
      orbital_inclination,
      rotation_direction,
      sidereal_orbital_period,
      orbital_period,
      rotation_period,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
