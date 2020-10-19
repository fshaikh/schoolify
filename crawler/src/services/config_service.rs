use crate::dal::objects::connection_details::ConnectionDetails;
use crate::models::crawler_config::CrawlerConfig;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::error::Error;
use crate::models::region_config::RegionConfig;
use crate::utils::file_utils::get_text_file_data;
use crate::utils::json_parser::deserialize;
use serde_json::Value; // App modules

pub fn get_crawler_request(region: &str) -> Result<CrawlerRequest, Error> {
    let platform_parsed_result = get_config_result(&String::from("./src/platform/config.json"))?;
    let config_parsed_result =
        get_config_result(&format!("./src/crawlers/{}/config.json", region))?;

    Ok(CrawlerRequest {
        region: RegionConfig::construct(&config_parsed_result),
        config: CrawlerConfig::construct(&config_parsed_result),
        database_config: ConnectionDetails::construct(&platform_parsed_result),
    })
}

fn get_config_result(config_path: &str) -> Result<Value, Error> {
    let config_result = get_text_file_data(config_path.to_string()).or_else(|err| {
        Err(Error {
            message: err.message,
        })
    })?;
    let result = deserialize(&config_result).or_else(|err| {
        Err(Error {
            message: "Error deserialzing config json".to_string(),
        })
    })?;
    Ok(result)
}
