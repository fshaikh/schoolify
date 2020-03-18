/// Base trait for all objects in crawler
extern crate chrono;
use crate::services::id_service::get_unique_id;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct ObjectBase {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub created_by: String,
    pub version: i32,
}

impl Default for ObjectBase {
    fn default() -> Self {
        ObjectBase {
            id: get_unique_id(),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            created_by: "".to_string(), // TODO
            version: 1,                 // TODO
        }
    }
}
