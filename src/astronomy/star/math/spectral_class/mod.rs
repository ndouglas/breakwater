use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::constants::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::temperature::ms_star_mass_to_temperature;

/// Get the spectral class of a main-sequence star in Kelvin based on its Msol.
#[named]
pub fn ms_star_mass_to_spectral_class(mass: f64) -> Result<String, Error> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
    return Err(Error::MassTooHighForMainSequence);
  }
  let temperature = ms_star_mass_to_temperature(mass)?;
  let spectral_type = match temperature {
    temperature if temperature < 3_700.0 => 'M',
    temperature if temperature < 5_200.0 => 'K',
    temperature if temperature < 6_000.0 => 'G',
    temperature if temperature < 7_500.0 => 'F',
    temperature if temperature < 10_000.0 => 'A',
    temperature if temperature < 33_000.0 => 'B',
    temperature if temperature < 95_000.0 => 'O',
    _ => unreachable!(),
  };
  let decile = match temperature {
    temperature if temperature < 3_700.0 => (10.0 * (1.0 - ((temperature - 2_000.0) / 1_700.0))),
    temperature if temperature < 5_200.0 => (10.0 * (1.0 - ((temperature - 3_700.0) / 1_500.0))),
    temperature if temperature < 6_000.0 => (10.0 * (1.0 - ((temperature - 5_200.0) / 800.0))),
    temperature if temperature < 7_500.0 => (10.0 * (1.0 - ((temperature - 6_000.0) / 1_500.0))),
    temperature if temperature < 10_000.0 => (10.0 * (1.0 - ((temperature - 7_500.0) / 2_500.0))),
    temperature if temperature < 33_000.0 => (10.0 * (1.0 - ((temperature - 10_000.0) / 23_000.0))),
    temperature if temperature < 95_000.0 => (10.0 * (1.0 - ((temperature - 33_000.0) / 62_000.0))),
    _ => unreachable!(),
  };
  let result = format!("{}{}V", spectral_type, decile);
  trace_var!(result);
  trace_exit!();
  Ok(result)
}
