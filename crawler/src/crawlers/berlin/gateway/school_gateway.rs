use crate::models::crawler_request::CrawlerRequest;
/// Gateway for all berlin school data
use crate::models::error::Error;
use crate::utils::http_client::{get, HttpRequest, HttpResponse};

/// Fetches the HTML for schools available at the root url
pub async fn fetch_schools_root(request: &CrawlerRequest) -> Result<String, Error> {
    let response = do_get(&request.config.schools.gateway.root).await;
    if response.status_code != 200 {
        let message = std::format!(
            "SchoolGateway::do_get. Error fetching for root for region: {}, {}",
            request.region,
            response.error
        );
        return Err(Error { message: message });
    }
    return Ok(response.result);
}

/// Fetches the HTML for school details available at details url
pub async fn fetch_school_details(request: &String) -> Result<String, Error> {
    let response = do_get(request).await;
    if response.status_code != 200 {
        let message = std::format!(
            "SchoolGateway::fetch_school_details. Error fetching for details for: {}, {}",
            request,
            response.error
        );
        return Err(Error { message: message });
    }
    return Ok(response.result);
}

async fn do_get(request: &String) -> HttpResponse {
    // make a HTTP GET call to fetch the html
    let http_request = HttpRequest {
        url: request.clone(),
        method: "GET".to_string(),
        body: "".to_string(),
    };

    return get(&http_request).await;
}
