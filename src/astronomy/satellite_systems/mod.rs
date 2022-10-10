use crate::astronomy::satellite_system::SatelliteSystem;

/// The `SatelliteSystems` object wraps a vector of `SatelliteSystem` objects.
#[derive(Clone, Debug, PartialEq)]
pub struct SatelliteSystems {
  /// SatelliteSystem objects.
  pub satellite_systems: Vec<SatelliteSystem>,
}
