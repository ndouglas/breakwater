use crate::astronomy::close_binary_star::CloseBinaryStar;
use crate::astronomy::star::Star;

pub mod constraints;
pub mod error;
use error::Error;

/// A `HostStar` is either a `Star` or a `CloseBinaryStar`.
///
/// This may seem counterintuitive, but a `CloseBinaryStar` is actually more
/// closely related to a `Star` than a `DistantBinaryStar`.  The reason for
/// this is that habitable planets can be in a circumbinary orbit around a
/// `CloseBinaryStar`, but can only be in an orbit around one member of a
/// `DistantBinaryStar`.  As a result, we handle `DistantBinaryStar` objects
/// with a distinct class.
#[derive(Clone, Debug, PartialEq)]
pub enum HostStar {
  /// A single star.
  Star(Star),
  /// A close binary star.
  CloseBinaryStar(CloseBinaryStar),
}

impl HostStar {

  /// Retrieve or calculate the total mass of the stars.
  ///
  /// Calculated in Msol.
  #[named]
  pub fn get_stellar_mass(&self) -> f64 {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => star.mass,
      CloseBinaryStar(close_binary_star) => close_binary_star.get_stellar_mass(),
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Retrieve or calculate the total number of stars in the system.
  #[named]
  pub fn get_stellar_count(&self) -> u8 {
    trace_enter!();
    use HostStar::*;
    let result = match &self {
      Star(star) => 1,
      CloseBinaryStar(close_binary_star) => 2,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this star is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    use HostStar::*;
    match &self {
      Star(star) => star.check_habitable()?,
      CloseBinaryStar(close_binary_star) => close_binary_star.check_habitable()?,
    }
    let result = Ok(());
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
