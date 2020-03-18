use crate::utils::json_parser::{get_boolean_value, get_string_value};
/// Data structure representing crawler config
use serde_json::Value;

#[derive(Debug)]
pub struct CrawlerConfigGateway {
    pub root: String,
    pub details: String,
}

#[derive(Debug)]
pub struct CrawlerConfigCache {
    pub file: String,
}

#[derive(Debug)]
pub struct CrawlerSchoolConfig {
    pub gateway: CrawlerConfigGateway,
    pub cache: CrawlerConfigCache,
    pub usecache: bool,
}

#[derive(Debug)]
pub struct CrawlerCatchmentAreaConfig {
    pub gateway: CrawlerConfigGateway,
    pub cache: CrawlerConfigCache,
    pub usecache: bool,
}

#[derive(Debug)]
pub struct CrawlerConfig {
    pub version: String,
    pub schools: CrawlerSchoolConfig,
    pub catchmentareas: CrawlerCatchmentAreaConfig,
}

impl CrawlerConfig {
    pub fn construct(v: Value) -> CrawlerConfig {
        return CrawlerConfig {
            version: get_string_value(&v["version"]),
            schools: CrawlerSchoolConfig {
                gateway: CrawlerConfigGateway {
                    root: get_string_value(&v["schools"]["gateway"]["root"]),
                    details: get_string_value(&v["schools"]["gateway"]["details"]),
                },
                cache: CrawlerConfigCache {
                    file: get_string_value(&v["schools"]["cache"]["file"]),
                },
                usecache: get_boolean_value(&v["schools"]["usecache"]),
            },
            catchmentareas: CrawlerCatchmentAreaConfig {
                gateway: CrawlerConfigGateway {
                    root: get_string_value(&v["catchmentareas"]["gateway"]["root"]),
                    details: get_string_value(&v["catchmentareas"]["gateway"]["details"]),
                },
                cache: CrawlerConfigCache {
                    file: get_string_value(&v["catchmentareas"]["cache"]["file"]),
                },
                usecache: get_boolean_value(&v["catchmentareas"]["usecache"]),
            },
        };
    }
}
