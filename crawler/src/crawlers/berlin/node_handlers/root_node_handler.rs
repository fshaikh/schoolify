use crate::crawlers::berlin::gateway::school_gateway::fetch_schools_root;
use crate::crawlers::berlin::models::root_result::RootResults;
use crate::crawlers::berlin::services::html_parser_adaptor::get_school_root_results;
/// Handler for the root node in the crawler graph
use crate::models::crawler_request::CrawlerRequest;
use crate::models::error::Error;
use crate::utils::file_utils::get_text_file_data;

pub async fn process_root(request: &CrawlerRequest) -> Result<RootResults, Error> {
    println!(
        "SCHOOL::CRAWLING - processing root begin: {}",
        request.region
    );
    // fetch the root result
    let root_html = get_root_html(&request).await?;
    println!(
        "SCHOOL::CRAWLING - Fetched root html for {}",
        request.region
    );
    // parse the html
    let root_result = get_schools_list(&root_html);
    println!("SCHOOL::CRAWLING - Parsed Root Html");

    match root_result {
        Ok(result) => {
            println!("{:?}", result.results);
            return Ok(result);
        }
        Err(err) => Err(err),
    }
}

/// Get the html from the root url. Invokes gateway to fetch the html
async fn get_root_html(request: &CrawlerRequest) -> Result<String, Error> {
    if request.config.schools.usecache {
        println!(
            "SCHOOL::CRAWLING - Fetching root html from file: {}",
            request.config.schools.cache.file
        );
        return get_root_html_from_file(request).await;
    }
    println!(
        "SCHOOL::CRAWLING - Fetching root html from gateway: {}",
        request.config.schools.gateway.root
    );
    return get_root_html_from_gateway(request).await;
}

async fn get_root_html_from_gateway(request: &CrawlerRequest) -> Result<String, Error> {
    let root_result = fetch_schools_root(&request).await;
    match root_result {
        Ok(html) => return Ok(html),
        Err(err) => {
            println!(
                "root_node_handler::get_root_html. Error fetching root HTML. {}",
                err.message
            );
            return Err(err);
        }
    }
}

async fn get_root_html_from_file(request: &CrawlerRequest) -> Result<String, Error> {
    let config_path = format!(
        "./.cache/{}/{}",
        request.region, request.config.schools.cache.file
    );

    get_text_file_data(config_path).or_else(|err| {
        Err(Error {
            message: err.message,
        })
    })
}

fn get_schools_list(root_html: &String) -> Result<RootResults, Error> {
    return get_school_root_results(root_html);
}
