use rand::prelude::*;

use crate::astronomy::AstronomicalError;
use crate::astronomy::RADIUS_OF_STELLAR_NEIGHBORHOOD;
use crate::astronomy::DENSITY_OF_STELLAR_NEIGHBORHOOD;

pub mod constraints;
pub use constraints::*;
pub mod stellar_neighbor;
pub use stellar_neighbor::*;

/// The `StellarNeighborhood` type.
///
/// This is mostly a container for star systems.
#[derive(Clone, Debug, PartialEq)]
pub struct StellarNeighborhood {
  /// The radius of this neighborhood, measured in light years.
  pub radius: f64,
  /// The stellar density of this neighborhood, measured in stars per cubic
  /// light year.  This is not terribly useful once the neighborhood has
  /// been generated, but we keep it around for posterity.
  pub density: f64,
  /// Stellar "neighbors", which is a glorified tuple of three-dimensional
  /// coordinates and a star system.
  pub neighbors: Vec<StellarNeighbor>,
}

impl StellarNeighborhood {

  /// Generate a random stellar neighborhood with the specified constraints.
  ///
  /// This may or may not be habitable.
  #[named]
  pub fn get_random_constrained<R: Rng + ?Sized>(
    rng: &mut R,
    constraints: &StellarNeighborhoodConstraints,
  ) -> Result<StellarNeighborhood, AstronomicalError> {
    trace_enter!();
    let radius = constraints.radius.unwrap_or(RADIUS_OF_STELLAR_NEIGHBORHOOD);
    let density = constraints.density.unwrap_or(DENSITY_OF_STELLAR_NEIGHBORHOOD);
    let neighbors = vec![];
    let result = StellarNeighborhood {
      radius,
      density,
      neighbors,
    };
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
  pub fn get_random() -> Result<(), AstronomicalError> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let constraints = StellarNeighborhoodConstraints {
      radius: Some(RADIUS_OF_STELLAR_NEIGHBORHOOD),
      density: Some(DENSITY_OF_STELLAR_NEIGHBORHOOD),
    };
    let stellar_neighborhood = StellarNeighborhood::get_random_constrained(&mut rng, &constraints)?;
    trace_var!(stellar_neighborhood);
    println!("{:#?}", stellar_neighborhood);
    trace_exit!();
    Ok(())
  }

}
