/// Base trait for all objects in crawler
extern crate chrono;
use chrono::{DateTime, Utc};

pub struct ObjectBase {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub created_by: String,
    pub version: i32,
}
