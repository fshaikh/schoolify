/// Represents a location in lat/lon format

pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn format(&self) -> String {
        return format!("Latitude: {}, Longitude: {}", self.latitude, self.longitude);
    }
}
