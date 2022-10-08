const METERS_PER_SOLAR_RADIUS: f64 = 149_597_870_700.0;
const METERS_PER_AU: f64 = 1.496E11;

pub fn solar_radii_to_meters(radii: f64) -> f64 {
  radii * METERS_PER_SOLAR_RADIUS
}

pub fn meters_to_solar_radii(meters: f64) -> f64 {
  meters / METERS_PER_SOLAR_RADIUS
}

pub fn au_to_meters(au: f64) -> f64 {
  au * METERS_PER_AU
}

pub fn meters_to_au(meters: f64) -> f64 {
  meters / METERS_PER_AU
}