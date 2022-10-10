use crate::astronomy::star_subsystem::constraints::Constraints as SubsystemConstraints;

/// Constraints for creating a star system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Star subsystem creation constraints.
  pub subsystem_constraints: Option<SubsystemConstraints>,
  /// Number of times to regenerate if requirements aren't met.
  pub retries: Option<u8>,
}

impl Constraints {
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::default());
    let retries = None;
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable_solitary());
    let retries = Some(10);
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable_close_binary() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable_close_binary());
    let retries = Some(10);
    Self {
      subsystem_constraints,
      retries,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable_distant_binary() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable_distant_binary());
    let retries = Some(10);
    Self {
      subsystem_constraints,
      retries,
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::default());
    let retries = None;
    Self {
      subsystem_constraints,
      retries,
    }
  }
}
