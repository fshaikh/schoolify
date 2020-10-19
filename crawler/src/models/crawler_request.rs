use crate::dal::objects::connection_details::ConnectionDetails;
/// Data structure for representing a crawler request.
use crate::models::crawler_config::CrawlerConfig;
use crate::models::region_config::RegionConfig;

#[derive(Debug)]
pub struct CrawlerRequest {
    /// Region for which the crawler is being run
    pub region: RegionConfig,
    pub config: CrawlerConfig,
    pub database_config: ConnectionDetails,
}
