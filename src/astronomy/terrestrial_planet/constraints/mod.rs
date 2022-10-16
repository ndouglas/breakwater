use rand::prelude::*;

use crate::astronomy::terrestrial_planet::constants::*;
use crate::astronomy::terrestrial_planet::error::Error;
use crate::astronomy::terrestrial_planet::math::temperature::get_equilibrium_temperature;
use crate::astronomy::terrestrial_planet::TerrestrialPlanet;

/// Constraints for creating a planet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass.
  pub minimum_mass: Option<f64>,
  /// The maximum mass.
  pub maximum_mass: Option<f64>,
  /// The minimum axial tilt.
  pub minimum_axial_tilt: Option<f64>,
  /// The maximum axial tilt.
  pub maximum_axial_tilt: Option<f64>,
  /// The minimum rotational period.
  pub minimum_rotational_period: Option<f64>,
  /// The maximum rotational period.
  pub maximum_rotational_period: Option<f64>,
  /// The distance from the host star, in AU.
  pub host_star_distance: Option<f64>,
  /// The luminosity of the host star, in Lsol.
  pub host_star_luminosity: Option<f64>,
}

impl Constraints {
  /// No constraints, just let it all hang out.
  fn habitable() -> Self {
    let minimum_mass = Some(MINIMUM_HABITABLE_MASS);
    let maximum_mass = Some(MAXIMUM_HABITABLE_MASS);
    let minimum_rotational_period = Some(MINIMUM_HABITABLE_ROTATIONAL_PERIOD);
    let maximum_rotational_period = Some(MAXIMUM_HABITABLE_ROTATIONAL_PERIOD);
    Self {
      minimum_mass,
      maximum_mass,
      minimum_rotational_period,
      maximum_rotational_period,
      ..Constraints::default()
    }
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<TerrestrialPlanet, Error> {
    trace_enter!();
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    trace_var!(minimum_mass);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    trace_var!(maximum_mass);
    let mass = rng.gen_range(minimum_mass..maximum_mass);
    trace_var!(mass);
    let mut result = TerrestrialPlanet::from_mass(mass)?;
    trace_var!(result);
    let minimum_axial_tilt = self.minimum_axial_tilt.unwrap_or(0.0);
    trace_var!(minimum_axial_tilt);
    let maximum_axial_tilt = self.maximum_axial_tilt.unwrap_or(180.0);
    trace_var!(maximum_axial_tilt);
    let axial_tilt = rng.gen_range(minimum_axial_tilt..maximum_axial_tilt);
    trace_var!(axial_tilt);
    result.axial_tilt = axial_tilt;
    result.tropic_zones = match axial_tilt {
      axial_tilt if axial_tilt < 90.0 => (0.0, axial_tilt),
      axial_tilt if axial_tilt > 90.0 => (0.0, 90.0 - (180.0 - axial_tilt)),
      _ => (0.0, 0.0),
    };
    result.polar_zones = match axial_tilt {
      axial_tilt if axial_tilt < 90.0 => ((90.0 - axial_tilt), 90.0),
      axial_tilt if axial_tilt > 90.0 => (90.0 - (180.0 - axial_tilt), 90.0),
      _ => (0.0, 0.0),
    };
    let bond_albedo = result.bond_albedo;
    trace_var!(bond_albedo);
    let greenhouse_effect = result.greenhouse_effect;
    trace_var!(greenhouse_effect);
    let host_star_luminosity = self.host_star_luminosity.unwrap_or(1.0);
    trace_var!(host_star_luminosity);
    let host_star_distance = self.host_star_distance.unwrap_or(1.0);
    trace_var!(host_star_distance);
    result.equilibrium_temperature =
      get_equilibrium_temperature(bond_albedo, greenhouse_effect, host_star_luminosity, host_star_distance);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
    let minimum_axial_tilt = None;
    let maximum_axial_tilt = None;
    let minimum_rotational_period = None;
    let maximum_rotational_period = None;
    let host_star_distance = None;
    let host_star_luminosity = None;
    Self {
      minimum_mass,
      maximum_mass,
      minimum_axial_tilt,
      maximum_axial_tilt,
      minimum_rotational_period,
      maximum_rotational_period,
      host_star_distance,
      host_star_luminosity,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let planet = &Constraints::default().generate(&mut rng)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }
}
