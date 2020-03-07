use crate::utils::json_parser::get_string_value;
/// Data structure representing crawler config
use serde_json::Value;

#[derive(Debug)]
pub struct CrawlerConfigGatewayData {
    pub root: String,
    pub details: String,
    pub file: String,
}

#[derive(Debug)]
pub struct CrawlerConfigGateway {
    pub schools: CrawlerConfigGatewayData,
    pub catchmentareas: CrawlerConfigGatewayData,
}

#[derive(Debug)]
pub struct CrawlerConfig {
    pub version: String,
    pub gateway: CrawlerConfigGateway,
}

impl CrawlerConfig {
    pub fn construct(v: Value) -> CrawlerConfig {
        return CrawlerConfig {
            version: get_string_value(&v["version"]),
            gateway: CrawlerConfigGateway {
                schools: CrawlerConfigGatewayData {
                    root: get_string_value(&v["gateway"]["schools"]["root"]),
                    details: get_string_value(&v["gateway"]["schools"]["details"]),
                    file: get_string_value(&v["gateway"]["schools"]["file"]),
                },
                catchmentareas: CrawlerConfigGatewayData {
                    root: get_string_value(&v["gateway"]["catchmentareas"]["root"]),
                    details: get_string_value(&v["gateway"]["catchmentareas"]["details"]),
                    file: get_string_value(&v["gateway"]["catchmentareas"]["file"]),
                },
            },
        };
    }
}
