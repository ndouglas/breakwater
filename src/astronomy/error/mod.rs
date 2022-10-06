/// The `AstronomicalError` type.
///
/// This is for errors concerning astronomy, e.g. generating a main-sequence
/// star that's too big, or a star without a habitable zone, etc.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AstronomicalError {
  /// The habitable zone is contained within the forbidden zone.
  HabitableZoneContainedWithinForbiddenZone,
  /// The habitable zone isn't sufficiently far from the host stars.
  HabitableZoneTooCloseToBinaryHostStars,
  /// No habitable conditions found anywhere in subsystem.
  NoHabitableZoneFoundInSubsystem,
}
