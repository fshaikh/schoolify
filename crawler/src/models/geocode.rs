use crate::models::location::Location;

pub struct GeocodeRequest {
    pub address: String,
}

pub struct GeocodeResponse {
    // Address(String),
    pub location: Location,
}

impl GeocodeResponse {
    pub fn format(&self) -> String {
        return format!("{}", self.location.format());
    }
}
