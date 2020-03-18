use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ForwardGeocodeResponse {
    pub resourceSets: Vec<ResourceSet>,
}

#[derive(Serialize, Deserialize)]
pub struct ResourceSet {
    pub resources: Vec<Resource>,
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub point: Point,
}

#[derive(Serialize, Deserialize)]
pub struct Point {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

pub fn parse(input: &String) -> Option<ForwardGeocodeResponse> {
    let deserialized_result: Result<ForwardGeocodeResponse, serde_json::Error> =
        serde_json::from_str(input);
    match deserialized_result {
        Ok(result) => {
            if result.resourceSets[0].resources.len() == 0 {
                println!(
                    "bing_geocoder_codec::parse - Error decoding Bing Geocoder response: {}",
                    input
                );
                return None;
            }
            return Some(result);
        }
        Err(err) => {
            println!(
                "bing_geocoder_codec::parse - Error decoding Bing Geocoder response: {}",
                err
            );
            return None;
        }
    };
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_decode_correctly() {
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
        let decode_response = parse(&input.to_string());
        assert_eq!(decode_response.is_none(), false);
        let decoded = decode_response.unwrap();
        assert_eq!(
            decoded.resourceSets[0].resources[0].point.coordinates[0],
            52.536488
        );
        assert_eq!(
            decoded.resourceSets[0].resources[0].point.coordinates[1],
            13.425927
        );
    }
}
//#endregion Tests

// we are expecting a JSON of the following form. So we need
// resourceSets[0].resources[0].point.coordinates
/*
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
}
*/
