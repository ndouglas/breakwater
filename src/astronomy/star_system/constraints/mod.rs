use crate::astronomy::star_system::Constraints as SubsystemConstraints;

/// Constraints for creating a star system.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StarSystemConstraints {
  /// Star subsystem creation constraints.
  pub star_subsystem_constraints: Option<SubsystemConstraints>,
}

impl StarSystemConstraints {
  /// Generate a main-sequence star system.
  pub fn main_sequence() -> Self {
    let star_subsystem_constraints = Some(SubsystemConstraints::main_sequence());
    Self {
      star_subsystem_constraints,
    }
  }

  /// Generate a habitable star system.
  pub fn habitable() -> Self {
    let star_subsystem_constraints = Some(SubsystemConstraints::habitable_solitary_or_p_type_binary());
    Self {
      star_subsystem_constraints,
    }
  }
}

impl Default for StarSystemConstraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let star_subsystem_constraints = Some(SubsystemConstraints::default());
    Self {
      star_subsystem_constraints,
    }
  }
}
