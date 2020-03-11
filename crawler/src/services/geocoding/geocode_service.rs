/// Geocoding features
use crate::models::geocode::{GeocodeRequest,GeocodeResponse};
use crate::models::error::Error;
use crate::platform::platform_traits::{IGeocoder};
use crate::services::geocoding::bing::bing_geocoder_service::{BingGeocoder};

/// Forward geocoding
pub async fn geocode(request: &GeocodeRequest) -> Result<GeocodeResponse,Error>{
    // call factory to get the geocoder to be used. 
    let geocoder = BingGeocoder::new();
    return geocoder.forward_geocode(request).await;
}

// fn create_geocoder() -> IGeocoder {
//     // all geocoders are defined in platform config. 
//     // all geocoders implement IGeocoder trait.

//     // For now, just return bing geocoder
//     return BingGeocoder::new();
// }