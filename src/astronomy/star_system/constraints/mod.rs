use crate::astronomy::star_system::subsystem::constraints::Constraints as SubsystemConstraints;

/// Constraints for creating a star system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Star subsystem creation constraints.
  pub subsystem_constraints: Option<SubsystemConstraints>,
}

impl Constraints {
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::main_sequence());
    Self {
      subsystem_constraints,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::habitable_solitary_or_p_type_binary());
    Self {
      subsystem_constraints,
    }
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let subsystem_constraints = Some(SubsystemConstraints::default());
    Self {
      subsystem_constraints,
    }
  }
}
