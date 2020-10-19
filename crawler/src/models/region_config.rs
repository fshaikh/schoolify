use crate::utils::json_parser::get_string_value;
use serde_json::Value;

#[derive(Debug)]
pub struct RegionConfig {
    pub id: String,
    pub key: String,
    pub name: String,
    pub country: String,
    pub continent: String,
}

impl RegionConfig {
    pub fn construct(v: &Value) -> RegionConfig {
        return RegionConfig {
            id: get_string_value(&v["region"]["id"]),
            key: get_string_value(&v["region"]["key"]),
            name: get_string_value(&v["region"]["name"]),
            country: get_string_value(&v["region"]["country"]),
            continent: get_string_value(&v["region"]["continent"]),
        };
    }
}
