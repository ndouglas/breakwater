pub const ERGS_PER_SEC_PER_LSOL: f64 = 3.826E33;
pub const JOULES_PER_SEC_PER_LSOL: f64 = 3.826E26;

/// ergs/sec -> Lsol
pub fn ergs_to_lsol(ergs: f64) -> f64 {
  ergs / ERGS_PER_SEC_PER_LSOL
}

/// Lsol -> ergs/sec
pub fn lsol_to_ergs(lsol: f64) -> f64 {
  lsol * ERGS_PER_SEC_PER_LSOL
}

/// J/sec -> Lsol
pub fn joules_to_lsol(joules: f64) -> f64 {
  joules / JOULES_PER_SEC_PER_LSOL
}

/// Lsol -> J/sec
pub fn lsol_to_joules(lsol: f64) -> f64 {
  lsol * JOULES_PER_SEC_PER_LSOL
}

/// W -> Lsol
pub fn watts_to_lsol(watts: f64) -> f64 {
  watts / JOULES_PER_SEC_PER_LSOL
}

/// Lsol -> W
pub fn lsol_to_watts(lsol: f64) -> f64 {
  lsol * JOULES_PER_SEC_PER_LSOL
}
