use geojson::{GeoJson, Geometry, Value};
extern crate chrono;
use chrono::{Utc};

use crate::utils::file_utils::{get_text_file_data, FileError};
use crate::utils::json_parser::get_string_value;
use crate::models::catchment_area::{CatchmentArea, GeometryValue};
use crate::models::error::{Error};
use crate::models::object_base::ObjectBase;
use crate::models::crawler_request::CrawlerRequest;

pub fn fetch_catchmentareas(request: &CrawlerRequest) -> Result<Vec<CatchmentArea>, Error> {
    // read the file which contains the geojson
    let geojson_value_result = get_geojson_value(request);
    return match geojson_value_result {
        Ok(geojson_str) => Ok(parse_geojson(&request.region, &geojson_str)),
        Err(_) => Err(Error {
            message: format!(
                "Unable to read/parse catchment area geojson: {}",
                request.region
            ),
        }),
    };
}

fn get_geojson_value(request: &CrawlerRequest) -> Result<String, FileError> {
    let config_path = format!(
        "./.cache/{}/{}",
        request.region, request.config.gateway.catchmentareas.file
    );
    return get_text_file_data(config_path);
}

/// Parses geojsont to read proeprties and geometry
/// GeoJSON structure:
///     GeoJson
///         FeatureCollection
///            feature
///              properties
///              geometry
///                 type (point, multipoint, linestring, multilinestring, polygon, multipolygon)
///                 coordinates

fn parse_geojson(region: &String, geojson_str: &String) -> Vec<CatchmentArea> {
    let geojson = geojson_str.parse::<GeoJson>().unwrap();
    let mut catchment_areas = Vec::new();
    match geojson {
        GeoJson::FeatureCollection(feature_collection) => {
            for feature in feature_collection.features {
                let properties = feature.properties.unwrap();
                let area_key = get_string_value(&properties["ESB"]);
                let district_name = get_string_value(&properties["BEZName"]);
                let district_key = get_string_value(&properties["BEZ"]);
                println!(
                    "BEZ:{}, BEZName: {}, ESB: {}",
                    area_key, district_name, district_key
                );
                let geometry = get_geometry_value(&feature.geometry.unwrap(),&area_key);
                catchment_areas.push(CatchmentArea {
                    meta: ObjectBase {
                        id: format!("{}:{}", region, area_key),
                        created_at: Utc::now(),
                        modified_at: Utc::now(),
                        created_by: "".to_string(),
                        version: 1,
                    },
                    area_key: area_key,
                    district_key: district_key,
                    district_name: district_name,
                    geometry: geometry,
                })
            }
        }
        _ => panic!("Failed to find feature collection in catchment area geojson"),
    };

    return catchment_areas;
}

fn get_geometry_value(geometry: &Geometry, area_key: &String) -> GeometryValue {
    match &geometry.value {
        Value::Polygon(polygon) => {
            println!("Found polygon for : {}", area_key);
            return crate::models::catchment_area::GeometryValue::Polygon(polygon.to_vec());
        },
        Value::MultiPolygon(polygons) => {
            println!("Found multi polygon for : {}", area_key);
            return crate::models::catchment_area::GeometryValue::MultiPolygon(polygons.to_vec());
        },
        _ => panic!(
            "Failed to find polygon geometry in catchment area geojson: {}",
            area_key
        ),
    };
}
