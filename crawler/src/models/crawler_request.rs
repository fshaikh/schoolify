/// Data structure for representing a crawler request.
use crate::models::crawler_config::CrawlerConfig;

#[derive(Debug)]
pub struct CrawlerRequest {
    /// Region for which the cralwer is being run
    pub region: String,
    pub config: CrawlerConfig,
}
