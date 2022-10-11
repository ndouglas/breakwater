use crate::astronomy::satellite_system::SatelliteSystem;

pub mod constants;
pub mod constraints;
pub mod error;

/// The `SatelliteSystems` object wraps a vector of `SatelliteSystem` objects.
#[derive(Clone, Debug, PartialEq)]
pub struct SatelliteSystems {
  /// SatelliteSystem objects.
  pub satellite_systems: Vec<SatelliteSystem>,
}
