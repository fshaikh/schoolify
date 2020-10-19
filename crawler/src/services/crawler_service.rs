use crate::cache::cache_service::save_crawler_response;
use crate::crawlers::berlin::services::crawler_service::CrawlerService;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::error::Error;
use crate::platform::platform_traits::ICrawler;

pub async fn crawl_region(request: &CrawlerRequest) -> Result<CrawlerResponse, Error> {
    if request.region.key == "berlin".to_string() {
        // construct berlin crawler service
        let crawler = CrawlerService::new();
        let crawler_response = do_crawl(&crawler, request).await?;
        // cache it
        save_crawler_response(request, &crawler_response);
        return Ok(crawler_response);
    } else {
        panic!("Unsupported region: {}", request.region.key);
    }
}

async fn do_crawl(
    crawler: &dyn ICrawler,
    request: &CrawlerRequest,
) -> Result<CrawlerResponse, Error> {
    println!("Calling crawler for region: {:?}", request);
    return crawler.crawl(request).await;
}

//TODO: Use this once you understand how to return trait objects
// pub fn create_crawler(request: &CrawlerRequest)-> dyn ICrawler {
//     if request.region == "berlin".to_string() {
//         // construct berlin crawler service
//         return CrawlerService::new();
//     } else {
//         panic!("Unsupported region: {}", request.region);
//     }
// }
