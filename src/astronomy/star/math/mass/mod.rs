const KG_PER_SOLAR_MASS: f64 = 1.989E30;

/// Msol -> KG
pub fn msol_to_kg(msol: f64) -> f64 {
  msol * KG_PER_SOLAR_MASS
}

/// KG -> Msol
pub fn kg_to_msol(kg: f64) -> f64 {
  kg / KG_PER_SOLAR_MASS
}
