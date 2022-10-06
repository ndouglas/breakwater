/// Indicates whether a subsystem is amenable to a habitable satellite.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StarSubsystemOrbitalInformationOrbitType {
  /// Neither component nor the subsystem as a whole is amenable.
  None,
  /// The subsystem as a whole can be orbited habitably.
  PType,
  /// The primary star/subsystem can be orbited habitably.
  STypePrimary,
  /// The secondary star/subsystem can be orbited habitably.
  STypeSecondary,
  /// Both primary and secondary stars/subsystems can be orbited habitably.
  STypeBoth,
}