use crate::cache::providers::file::file_cache_provider::FileCacheProvider;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::platform::platform_traits::ICacheProvider;
// use crate::utils::binary_serde::{serialize};

pub fn save_crawler_response(request: &CrawlerRequest, response: &CrawlerResponse) {
    // serialize to compact binary format
    let encoded = bincode::serialize(response).unwrap();
    println!("cache_service::save_crawler_response - Successfully serialized");
    // send to cache provider : TODO: use file by default, later pick from platform config
    let result = save_to_file_cache(request, &encoded);
    if result {
        println!("Saved crawler response to cache")
    } else {
        println!("Failed to save crawler response to cache")
    }
}

pub fn save_to_file_cache(request: &CrawlerRequest, encoded: &Vec<u8>) -> bool {
    let cache_provider = FileCacheProvider::new();
    // save to .cache/{region}/crawler_response.sm
    let key = format!(".cache/{}/crawler_response.sm", request.region.key);
    println!("Caching crawler response to : {}", key);
    save_to_cache(&cache_provider, &key, encoded)
}

fn save_to_cache(provider: &dyn ICacheProvider, key: &String, value: &Vec<u8>) -> bool {
    provider.set(key, value)
}

// println!("Serialized: {:?}", encoded);
// let cr = bincode::deserialize::<CrawlerResponse>(&encoded);
// println!("Deserialized: {:?}",cr);
