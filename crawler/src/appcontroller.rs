use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::error::Error;
/// Application Controller. Entry point into the application
use crate::services::cli_parser::{parse, read_args};
use crate::services::crawler_service::crawl_region;
use crate::services::statistics_service::get_statistics_formatted;

pub async fn start() -> Result<bool, Error> {
    // Parse the CLI arguments and handle the response
    let parser_response = parse(read_args());
    match parser_response {
        Ok(crawler_request) => {
            let crawler_response_result = process_crawl(&crawler_request).await;
            match crawler_response_result {
                Ok(crawler_response) => {
                    println!("Successfully finished crawling");
                    crate::services::database_service::handle_db_operations(
                        &crawler_request,
                        &crawler_response,
                    )
                }
                Err(err) => Err(err),
            }
        }
        Err(err) => {
            println!("Parser Error: {}", err.message);
            let error = Error {
                message: err.message,
            };
            Err(error)
        }
    }
}

async fn process_crawl(request: &CrawlerRequest) -> Result<CrawlerResponse, Error> {
    println!("Crawler Region: {}", request.region.key);
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
        println!("Invalid region: {}", request.region.key);
        return false;
    }
    return true;
}
