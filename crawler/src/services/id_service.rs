use uuid::Uuid;

pub fn get_unique_id() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}

pub fn parse_uuid(id: &str) -> Uuid {
    Uuid::parse_str(id).unwrap()
}
