use crate::astronomy::AstronomicalError;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

/// Get the luminosity of a main-sequence star in Lsol based on its Msol.
#[named]
pub fn get_main_sequence_luminosity_from_mass(mass: f64) -> Result<f64, AstronomicalError> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
    return Err(AstronomicalError::StellarMassTooLowForMainSequence);
  }
  if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
    return Err(AstronomicalError::StellarMassTooHighForMainSequence);
  }
  let result = match mass {
    mass if mass < 0.43 => 0.23 * mass.powf(2.3),
    mass if mass < 2.0 => mass.powf(4.0),
    mass if mass < 55.0 => 1.4 * mass.powf(3.5),
    mass if mass < MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND => 32_000.0 * mass,
    _ => unreachable!(),
  };
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
  pub fn test_get_main_sequence_luminosity_from_mass() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = 1.0;
    let mut actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual);
    // M1V
    mass = 0.40;
    expected = 0.028;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // K9V
    mass = 0.50;
    expected = 0.063;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // G7V
    mass = 0.90;
    expected = 0.656;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // F6V
    mass = 1.20;
    expected = 2.073;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // A6V
    mass = 1.70;
    expected = 8.352;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // A6V
    mass = 1.70;
    expected = 8.352;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // B5V
    mass = 8.0;
    expected = 2027.4;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // O8V
    mass = 25.0;
    expected = 109375.0;
    actual = get_main_sequence_luminosity_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    trace_exit!();
    Ok(())
  }
}
