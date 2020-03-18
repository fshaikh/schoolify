use crate::crawlers::berlin::gateway::school_gateway::fetch_school_details;
use crate::crawlers::berlin::models::details_result::DetailsResult;
use crate::crawlers::berlin::models::root_result::RootResult;
use crate::crawlers::berlin::services::html_parser_adaptor::get_school_details_result;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::error::Error;

/// Handler for the details node in the crawler graph
pub async fn process(
    request: &CrawlerRequest,
    root_result: &RootResult,
) -> Result<DetailsResult, Error> {
    // construct details url from root_result and request
    let details_url = construct_details_url(request, root_result);

    println!(
        "SCHOOL::CRAWLING - Fetching details from gateway for: {}",
        root_result.school_name
    );
    // call school gateway to get the details html
    let details_html_result = fetch_school_details(&details_url).await?;
    println!(
        "SCHOOL::CRAWLING - Fetched details from gateway for: {}",
        root_result.school_name
    );

    // parse the html using html parser adaptor
    println!(
        "SCHOOL::CRAWLING - Parsing details HTML: {}",
        root_result.school_name
    );
    return get_school_details_result(root_result, &details_html_result);
}

fn construct_details_url(request: &CrawlerRequest, root_result: &RootResult) -> String {
    return format!("{}", root_result.details_url);
}

// https://www.berlin.de/sen/bildung/schule/berliner-schulen/schulverzeichnis/Schulportrait.aspx?IDSchulzweig=%2020307
// https://www.berlin.de/sen/bildung/schule/berliner-schulen/schulverzeichnis/
// Schulportrait.aspx?IDSchulzweig= 20248
