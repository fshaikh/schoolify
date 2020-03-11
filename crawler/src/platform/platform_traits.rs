/// Defines traits which all region crawlers must implement
///
use async_trait::async_trait;

use crate::models::error::Error;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::geocode::{GeocodeRequest,GeocodeResponse};

#[async_trait]
pub trait ICrawler {
    async fn crawl(&self, request: &CrawlerRequest) -> Result<CrawlerResponse, Error>;
}

#[async_trait]
pub trait IGeocoder {
    async fn forward_geocode(&self, request: &GeocodeRequest) -> Result<GeocodeResponse,Error>;
}
