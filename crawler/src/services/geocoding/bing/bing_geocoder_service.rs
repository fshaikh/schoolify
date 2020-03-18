use async_trait::async_trait;

use crate::models::error::Error;
use crate::models::geocode::{GeocodeRequest, GeocodeResponse};
use crate::platform::platform_traits::IGeocoder;
use crate::services::geocoding::bing::bing_geocoder_gateway::{
    fetch_geocode_location, BingGeocodeGatewayRequest,
};
use crate::services::geocoding::bing::bing_geocoder_mapper::map;

pub struct BingGeocoder {}

impl BingGeocoder {
    pub fn new() -> BingGeocoder {
        BingGeocoder {}
    }
}

#[async_trait]
impl IGeocoder for BingGeocoder {
    async fn forward_geocode(&self, request: &GeocodeRequest) -> Result<GeocodeResponse, Error> {
        let address = request.address.clone();
        println!("Forward geocoding for: {}", address);
        // call gateway
        let gateway_request = BingGeocodeGatewayRequest { address: address };
        let gateway_response = fetch_geocode_location(&gateway_request).await;
        if gateway_response.is_none() {
            let error = "bing_geocoder_service::forward_geocode - Error from gateway".to_string();
            println!("{}", error);
            return Err(Error { message: error });
        }
        let unwrapped_response = gateway_response.unwrap();
        // call mapper
        return map(&unwrapped_response);
    }
}
