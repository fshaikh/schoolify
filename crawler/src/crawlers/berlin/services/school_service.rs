use crate::crawlers::berlin::models::school_results::{SchoolResult, SchoolResults};
use crate::crawlers::berlin::node_handlers::details_node_handler::process;
use crate::crawlers::berlin::node_handlers::root_node_handler::process_root;
use crate::models::error::Error;
use crate::models::crawler_request::CrawlerRequest;

/// Service to crawl schools for a region. Visits each node of the crawler graph and processes the results
pub async fn fetch_schools(request: &CrawlerRequest) -> Result<SchoolResults, Vec<Error>> {
    let root_results_result = process_root(request).await;
    let mut errors: Vec<Error> = Vec::new();

    if root_results_result.is_err() {
        errors.push(root_results_result.err().unwrap());
        return Err(errors);
    }
    let mut school_results = SchoolResults::new();
    let root_results = root_results_result.unwrap();

    for root_result in root_results.results {
        println!(
            "Processing root result for details crawling: {}",
            root_result.id
        );
        let details_result = process(request, &root_result).await;
        match details_result {
            Ok(details) => {
                println!("Details: {}", details.format());
                school_results.add_result(SchoolResult {
                    root_result: root_result,
                    details_result: details,
                });
            }
            Err(err) => errors.push(err),
        }
    }
    return Ok(school_results);
}
