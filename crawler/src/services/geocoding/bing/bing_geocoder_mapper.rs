use crate::models::error::Error;
use crate::models::geocode::GeocodeResponse;
use crate::models::location::Location;
use crate::services::geocoding::bing::bing_geocoder_codecs::parse;

pub fn map(response: &String) -> Result<GeocodeResponse, Error> {
    let parsed_response = parse(response);
    if parsed_response.is_none() {
        return Err(Error {
            message: "bing_geocoder_mapper::map - Unable to parse ForwardGeocodeResponse"
                .to_string(),
        });
    }

    let forward_geocode = parsed_response.unwrap();
    let location = Location {
        latitude: forward_geocode.resourceSets[0].resources[0]
            .point
            .coordinates[0],
        longitude: forward_geocode.resourceSets[0].resources[0]
            .point
            .coordinates[1],
    };

    let geocode_response = GeocodeResponse { location: location };
    return Ok(geocode_response);
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::location::Location;

    #[test]
    fn it_should_map_correctly() {
        let input = r#"
        {
            "authenticationResultCode": "ValidCredentials",
            "brandLogoUri": "",
            "copyright": "",
            "resourceSets": [
                {
                    "estimatedTotal": 1,
                    "resources": [
                        {
                            "__type": "Location:http://schemas.microsoft.com/search/local/ws/rest/v1",
                            "bbox": [
                                52.532625282429322,
                                13.417460439096212,
                                52.540350717570675,
                                13.434393560903787
                            ],
                            "name": "Christburger Strasse 41, Berlin, Berlin 10405, Germany",
                            "point": {
                                "type": "Point",
                                "coordinates": [
                                    52.536488,
                                    13.425927
                                ]
                            },
                            "address": {
                                "addressLine": "Christburger Strasse 41",
                                "adminDistrict": "Berlin",
                                "adminDistrict2": "Berlin",
                                "countryRegion": "Germany",
                                "formattedAddress": "Christburger Strasse 41, Berlin, Berlin 10405, Germany",
                                "locality": "Berlin",
                                "postalCode": "10405"
                            },
                            "confidence": "Medium",
                            "entityType": "Address",
                            "geocodePoints": [
                                {
                                    "type": "Point",
                                    "coordinates": [
                                        52.536488,
                                        13.425927
                                    ],
                                    "calculationMethod": "Rooftop",
                                    "usageTypes": [
                                        "Display"
                                    ]
                                },
                                {
                                    "type": "Point",
                                    "coordinates": [
                                        52.5362447103027,
                                        13.4257016386223
                                    ],
                                    "calculationMethod": "Rooftop",
                                    "usageTypes": [
                                        "Route"
                                    ]
                                }
                            ],
                            "matchCodes": [
                                "Good"
                            ]
                        }
                    ]
                }
            ],
            "statusCode": 200,
            "statusDescription": "OK",
            "traceId": ""
        }"#;
        let mapped_result = map(&input.to_string());
        assert_eq!(mapped_result.is_err(), false);
        let geocode_response = mapped_result.unwrap();
        assert_eq!(geocode_response.location.latitude, 52.536488);
        assert_eq!(geocode_response.location.longitude, 13.425927);
    }
}
//#endregion Tests
