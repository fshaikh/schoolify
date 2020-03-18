use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::error::Error;
/// Application Controller. Entry point into the application
use crate::services::cli_parser::{parse, read_args};
use crate::services::crawler_service::crawl_region;
use crate::services::statistics_service::get_statistics_formatted;

pub async fn start() -> Result<CrawlerResponse, Error> {
    // Parse the CLI arguments and handle the response
    let parser_response = parse(read_args());
    return match parser_response {
        Ok(crawler_request) => process_crawl(&crawler_request).await,
        Err(err) => {
            println!("Parser Error: {}", err.message);
            let error = Error {
                message: err.message,
            };
            return Err(error);
        }
    };
}

async fn process_crawl(request: &CrawlerRequest) -> Result<CrawlerResponse, Error> {
    println!("Crawler Region: {}", request.region);
    if is_valid_region(request) == false {
        let error = Error {
            message: String::from("invalid region"),
        };
        return Err(error);
    }

    // call crawler service
    let response = crawl_region(request).await;
    println!(
        "Crawling Completed. Statistics: {}",
        get_statistics_formatted()
    );
    return response;
}

fn is_valid_region(request: &CrawlerRequest) -> bool {
    let is_valid_region = crate::services::region_service::is_valid_region(request);
    if is_valid_region == false {
        println!("Invalid region: {}", request.region);
        return false;
    }
    return true;
}
