/// Entry point for the berlin region crawling. Implements traits as defined by the crawler platform
///
use async_trait::async_trait;

use crate::crawlers::berlin::models::school_results::SchoolResults;
use crate::crawlers::berlin::services::catchment_service::fetch_catchmentareas;
use crate::crawlers::berlin::services::school_service::fetch_schools;
use crate::platform::platform_traits::ICrawler;
use crate::models::catchment_area::CatchmentArea;
use crate::models::error::{make_error_message, Error};
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;

pub struct CrawlerService {}

impl CrawlerService {
    pub fn new() -> CrawlerService {
        CrawlerService {}
    }

    async fn crawl_schools(&self, request: &CrawlerRequest) -> SchoolResults {
        let school_results = fetch_schools(request).await;
        return match school_results {
            Ok(schools) => schools,
            Err(err) => panic!(make_error_message(&err)),
        };
    }

    fn crawl_catchmentareas(&self, request: &CrawlerRequest) -> Vec<CatchmentArea> {
        let catchmentareas_result = fetch_catchmentareas(request);
        return match catchmentareas_result {
            Ok(catchment_areas) => catchment_areas,
            Err(err) => panic!(err.message),
        };
    }

    
}

#[async_trait]
impl ICrawler for CrawlerService {
    /// Cralwing begins here for this region
    async fn crawl(&self, request: &CrawlerRequest) -> Result<CrawlerResponse, Error> {
        // 1.  crawl schools
        let _schools_result = self.crawl_schools(request).await;

        // 2.  crawl catchment areas
        let _catchment_areas_result = self.crawl_catchmentareas(request);

        // 3.  call mapper
        // let mapper_response = self.map(schools_results, catchment_areas_result);
        // 4. return results

        return Err(Error {
            message: "".to_string(),
        });
    }
}
