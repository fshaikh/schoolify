use crate::crawlers::berlin::models::details_result::DetailsResult;
use crate::crawlers::berlin::models::root_result::RootResult;
use crate::crawlers::berlin::models::school_results::{SchoolResult, SchoolResults};
use crate::crawlers::berlin::node_handlers::details_node_handler::process;
use crate::crawlers::berlin::node_handlers::root_node_handler::process_root;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::error::Error;
use crate::models::geocode::{GeocodeRequest, GeocodeResponse};
use crate::services::geocoding::geocode_service::geocode;
use crate::services::statistics_service::{record_school_count, record_school_crawl_time};
use std::time::Instant;

/// Service to crawl schools for a region. Visits each node of the crawler graph and processes the results
pub async fn fetch_schools(request: &CrawlerRequest) -> Result<SchoolResults, Vec<Error>> {
    let now = Instant::now();
    let results = fetch_schools_core(request).await;
    record_school_crawl_time(now.elapsed().as_secs_f64());
    return results;
}

async fn fetch_schools_core(request: &CrawlerRequest) -> Result<SchoolResults, Vec<Error>> {
    let mut errors: Vec<Error> = Vec::new();
    let mut school_results = SchoolResults::new();

    println!(
        "SCHOOL::CRAWLING - Crawling schools for: {}",
        request.region
    );
    let root_results_result = process_root(request).await;

    match root_results_result {
        Ok(root_results) => {
            record_school_count(root_results.results.len() as u64);

            for root_result in root_results.results {
                println!(
                    "Processing root result for details crawling: {}",
                    root_result.id
                );
                let details_result = process_details(request, &root_result).await;
                match details_result {
                    Ok(details) => school_results.add_result(SchoolResult {
                        root_result: root_result,
                        details_result: details,
                    }),
                    Err(err) => errors.push(err),
                }
            }
            return Ok(school_results);
        }
        Err(err) => {
            println!("SCHOOL::CRAWLING - Error Crawling root for: {:?}", err);
            errors.push(err);
            return Err(errors);
        }
    }
}

async fn process_details(
    request: &CrawlerRequest,
    root_result: &RootResult,
) -> Result<DetailsResult, Error> {
    let details_result = process(request, &root_result).await;
    println!(
        "SCHOOL::CRAWLING - Finished Parsing details HTML: {}",
        root_result.school_name
    );

    match details_result {
        Ok(mut details) => {
            println!(
                "SCHOOL::CRAWLING - Geocoding for : {}",
                root_result.school_name
            );
            let geocode_response_result = do_geocode(&details).await;

            match geocode_response_result {
                Ok(geocode_response) => details.set_location(geocode_response.location),
                _ => {}
            }
            println!("Details: {}", details.format());
            Ok(details)
        }
        Err(err) => {
            println!("Error processing details node: {:?}", err);
            return Err(err);
        }
    }
}

async fn do_geocode(details: &DetailsResult) -> Result<GeocodeResponse, Error> {
    let geocode_request = GeocodeRequest {
        address: details.get_address(),
    };
    return geocode(&geocode_request).await;
}
