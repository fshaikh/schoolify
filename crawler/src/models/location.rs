/// Represents a location in lat/lon format
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn format(&self) -> String {
        return format!("Latitude: {}, Longitude: {}", self.latitude, self.longitude);
    }
}

impl Clone for Location {
    fn clone(&self) -> Location {
        Location {
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}
