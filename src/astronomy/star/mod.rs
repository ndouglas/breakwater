use rand::prelude::*;

use crate::astronomy::AstronomicalError;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

pub mod luminosity;
pub use luminosity::*;
pub mod radius;
pub use radius::*;
pub mod spectral_class;
pub use spectral_class::*;
pub mod temperature;
pub use temperature::*;

/// The `Star` type.
///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Star {
  /// Type, Decile, Luminosity class.
  pub class: SpectralClass,
  /// Measured in Msol.
  pub mass: f64,
  /// Measured in Kelvin.
  pub temperature: f64,
  /// Measured in Rsol.
  pub radius: f64,
  /// Measured in Lsol.
  pub luminosity: f64,
  /// Measured in Gyr.
  pub life_expectancy: f64,
  /// Measured in Dsol.
  pub density: f64,
  /// Habitable zone, measured in AU.
  pub habitable_zone: (f64, f64),
}

/// Implementation of SpectralClass.
impl Star {
  /// From mass, for a main-sequence star.
  #[named]
  pub fn get_main_sequence_from_mass(mass: f64) -> Result<Star, AstronomicalError> {
    trace_enter!();
    let temperature = get_main_sequence_temperature_from_mass(mass)?;
    trace_var!(temperature);
    let luminosity = get_main_sequence_luminosity_from_mass(mass)?;
    trace_var!(luminosity);
    let radius = get_main_sequence_radius_from_mass(mass)?;
    trace_var!(radius);
    let class = SpectralClass::get_main_sequence_from_mass(mass)?;
    trace_var!(class);
    let life_expectancy = mass / luminosity * 10.0;
    trace_var!(life_expectancy);
    let density = mass / radius.powf(3.0);
    trace_var!(density);
    let habitable_zone = ((luminosity / 1.1).sqrt(), (luminosity / 0.53).sqrt());
    let result = Star {
      class,
      mass,
      luminosity,
      radius,
      temperature,
      life_expectancy,
      density,
      habitable_zone,
    };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Implement weighted distribution.
  #[named]
  pub fn get_random<R: Rng + ?Sized>(rng: &mut R) -> Result<Star, AstronomicalError> {
    trace_enter!();
    let mass = rng.gen_range(0.5..5.0);
    trace_var!(mass);
    let result = Star::get_main_sequence_from_mass(mass)?;
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
  pub fn get_random() {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let star = Star::get_random(&mut rng);
    trace_var!(star);
    println!("{:#?}", star);
    trace_exit!();
  }
}
