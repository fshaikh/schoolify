use crate::crawlers::berlin::services::crawler_service::CrawlerService;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::error::Error;
use crate::platform::platform_traits::ICrawler;

pub async fn crawl_region(request: &CrawlerRequest) -> Result<CrawlerResponse, Error> {
    if request.region == "berlin".to_string() {
        // construct berlin crawler service
        let crawler = CrawlerService::new();
        return do_crawl(&crawler, request).await;
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

//TODO: Use this once you understand how to return trait objects
// pub fn create_crawler(request: &CrawlerRequest)-> dyn ICrawler {
//     if request.region == "berlin".to_string() {
//         // construct berlin crawler service
//         return CrawlerService::new();
//     } else {
//         panic!("Unsupported region: {}", request.region);
//     }
// }
