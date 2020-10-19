use crate::platform::platform_traits::ICacheProvider;
use std::fs;
pub struct FileCacheProvider {
    // location: String;
}

impl FileCacheProvider {
    pub fn new() -> FileCacheProvider {
        FileCacheProvider {}
    }
}

// #[async_trait]
impl ICacheProvider for FileCacheProvider {
    fn set(&self, key: &str, value: &Vec<u8>) -> bool {
        let result = fs::write(key, value);
        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    // async fn get(key: &str): Option<V>{

    // }
}
