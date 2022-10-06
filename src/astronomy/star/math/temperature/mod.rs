use crate::astronomy::star::math::luminosity::get_main_sequence_star_luminosity_from_mass;
use crate::astronomy::star::math::radius::get_main_sequence_star_radius_from_mass;
use crate::astronomy::AstronomicalError;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

/// Get the temperature of a main-sequence star in Kelvin based on its Msol.
#[named]
pub fn get_main_sequence_star_temperature_from_mass(mass: f64) -> Result<f64, AstronomicalError> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
    return Err(AstronomicalError::StellarMassTooLowForMainSequence);
  }
  if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
    return Err(AstronomicalError::StellarMassTooHighForMainSequence);
  }
  let luminosity = get_main_sequence_star_luminosity_from_mass(mass)?;
  let radius = get_main_sequence_star_radius_from_mass(mass)?;
  let result = (luminosity / radius.powf(2.0)).powf(0.25) * 5776.0;
  trace_var!(result);
  trace_exit!();
  Ok(result)
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_get_main_sequence_star_temperature_from_mass() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = 5776.0;
    let mut actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // M1V
    mass = 0.40;
    expected = 3407.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // K9V
    mass = 0.50;
    expected = 3811.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // G7V
    mass = 0.90;
    expected = 5422.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // F6V
    mass = 1.20;
    expected = 6580.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // A6V
    mass = 1.70;
    expected = 8441.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // B5V
    mass = 8.0;
    expected = 21428.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // O8V
    mass = 25.0;
    expected = 41970.0;
    actual = get_main_sequence_star_temperature_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    trace_exit!();
    Ok(())
  }
}
