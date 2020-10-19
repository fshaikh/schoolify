use crate::utils::json_parser::{get_number_value, get_string_value};
use serde_json::Value;

#[derive(Debug)]
pub struct ConnectionDetails {
    pub host: String,
    pub port: u64,
    pub user_name: String,
    pub password: String,
    pub database: String,
}

impl ConnectionDetails {
    pub fn construct(v: &Value) -> ConnectionDetails {
        return ConnectionDetails {
            host: get_string_value(&v["database"]["connection"]["host"]),
            port: get_number_value(&v["database"]["connection"]["port"]),
            user_name: get_string_value(&v["database"]["connection"]["user_name"]),
            password: get_string_value(&v["database"]["connection"]["password"]),
            database: get_string_value(&v["database"]["connection"]["database_name"]),
        };
    }
}
