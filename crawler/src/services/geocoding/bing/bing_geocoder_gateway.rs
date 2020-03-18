/// Gateway for bing geocoding
// TODO: Later these will be added to platform config and exposed through a config service
use crate::utils::http_client::{encode_url, get, HttpRequest};

const GEOCODE_URL: &str = "https://dev.virtualearth.net/REST/v1/Locations?";
// Do not commit
const API_KEY: &str = "";

pub struct BingGeocodeGatewayRequest {
    pub address: String,
}

pub async fn fetch_geocode_location(request: &BingGeocodeGatewayRequest) -> Option<String> {
    let url = construct_forward_geocode_url(request);
    println!("Geocode url: {}", url);
    let http_request = HttpRequest {
        url: url,
        method: "GET".to_string(),
        body: "".to_string(),
    };
    let response = get(&http_request).await;
    if response.status_code != 200 {
        // what do we return
        println!(
            "bing_geocoder_gateway::fetch_geocode_location - Error in forward geocoding: {}, {}",
            request.address, response.status_code
        );
        return None;
    }
    println!("Geocode response: {}", response.result);
    return Some(response.result);
}

fn construct_forward_geocode_url(request: &BingGeocodeGatewayRequest) -> String {
    // key={}&addressLine={}
    let url = format!(
        "{}key={}&addressLine={}",
        GEOCODE_URL, API_KEY, request.address
    );
    // encode the url as address can contain spaces. For now simpl
    return encode_url(&url);
}
