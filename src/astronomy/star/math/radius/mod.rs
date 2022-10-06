use crate::astronomy::star::error::Error;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND;
use crate::astronomy::MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND;

/// Get the radius of a main-sequence star in Rsol based on its Msol.
#[named]
pub fn get_main_sequence_star_radius_from_mass(mass: f64) -> Result<f64, Error> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND {
    return Err(Error::MassTooHighForMainSequence);
  }
  let result = match mass {
    mass if mass < 1.0 => mass.powf(0.80),
    mass if mass >= 1.0 => mass.powf(0.57),
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
  pub fn test_get_main_sequence_star_radius_from_mass() -> Result<(), Error> {
    init();
    trace_enter!();
    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = 1.0;
    let mut actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual);
    // M1V
    mass = 0.40;
    expected = 0.480;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // K9V
    mass = 0.50;
    expected = 0.574;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // G7V
    mass = 0.90;
    expected = 0.919;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // F6V
    mass = 1.20;
    expected = 1.110;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // A6V
    mass = 1.70;
    expected = 1.353;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1e-3f64);
    // B5V
    mass = 8.0;
    expected = 3.272;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    // O8V
    mass = 25.0;
    expected = 6.264;
    actual = get_main_sequence_star_radius_from_mass(mass)?;
    assert_approx_eq!(expected, actual, 1f64);
    trace_exit!();
    Ok(())
  }
}
