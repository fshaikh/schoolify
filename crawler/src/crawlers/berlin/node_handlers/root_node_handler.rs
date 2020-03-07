use crate::crawlers::berlin::gateway::school_gateway::fetch_schools_root;
use crate::crawlers::berlin::models::root_result::RootResults;
use crate::crawlers::berlin::services::html_parser_adaptor::get_school_root_results;
use crate::models::error::Error;
/// Handler for the root node in the crawler graph
use crate::models::crawler_request::CrawlerRequest;

pub async fn process_root(request: &CrawlerRequest) -> Result<RootResults, Error> {
    // fetch the root result
    let root_html = get_root_html(&request).await?;
    // parse the html
    let root_result = get_schools_list(&root_html);

    match root_result {
        Ok(result) => {
            println!("{:?}", result.results);
            return Ok(result);
        }
        Err(err) => return Err(err),
    }
}

/// Get the html from the root url. Invokes gateway to fetch the html
async fn get_root_html(request: &CrawlerRequest) -> Result<String, Error> {
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

fn get_schools_list(root_html: &String) -> Result<RootResults, Error> {
    return get_school_root_results(root_html);
}
