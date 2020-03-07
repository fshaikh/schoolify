/// Handles JSON parsing
use serde_json::{Result, Value};

pub fn deserialize(data: &String) -> Result<Value> {
    return serde_json::from_str(data);
}

/// Get string value from Value enum
pub fn get_string_value(value: &Value) -> String {
    return match value {
        // Using to_owned() as val is &String. Cloning a reference yields a reference.
        // to_owned() returns the desired datatype.
        Value::String(val) => val.to_owned(),
        _ => panic!("Invalid config value. Unable to match to string value"),
    };
}
