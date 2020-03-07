use crate::crawlers::berlin::services::crawler_service::CrawlerService;
use crate::platform::platform_traits::ICrawler;
use crate::models::error::Error;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;

pub async fn crawl_region(request: &CrawlerRequest) -> Result<CrawlerResponse, Error> {
    if request.region == "berlin".to_string() {
        // construct berlin crawler service
        let berlin_crawler = CrawlerService::new();
        return do_crawl(&berlin_crawler, request).await;
    } else {
        panic!("Unsupported region: {}", request.region);
    }
}

async fn do_crawl(
    crawler: &dyn ICrawler,
    request: &CrawlerRequest,
) -> Result<CrawlerResponse, Error> {
    println!("Calling crawler for region: {:?}", request);
    return crawler.crawl(request).await;
}
