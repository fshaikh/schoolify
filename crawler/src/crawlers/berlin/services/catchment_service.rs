use geojson::{GeoJson, Geometry, Value};
use std::time::Instant;

use crate::models::catchment_area::{CatchmentArea, GeometryValue};
use crate::models::crawler_request::CrawlerRequest;
use crate::models::error::Error;
use crate::services::statistics_service::{
    record_catchment_area_count, record_catchment_area_crawl_time,
};
use crate::utils::file_utils::{get_text_file_data, FileError};
use crate::utils::json_parser::get_string_value;

pub fn fetch_catchmentareas(request: &CrawlerRequest) -> Result<Vec<CatchmentArea>, Error> {
    let now = Instant::now();
    let results = fetch_catchmentareas_core(request);
    record_catchment_area_crawl_time(now.elapsed().as_secs_f64());
    return results;
}

pub fn fetch_catchmentareas_core(request: &CrawlerRequest) -> Result<Vec<CatchmentArea>, Error> {
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
        request.region, request.config.catchmentareas.cache.file
    );
    return get_text_file_data(config_path);
}

/// Parses geojsont to read proeprties and geometry
/// GeoJSON structure:
///     GeoJson
///         FeatureCollection
///            feature
///              type
///              properties
///              geometry
///                 type (point, multipoint, linestring, multilinestring, polygon, multipolygon)
///                 coordinates

fn parse_geojson(_region: &String, geojson_str: &String) -> Vec<CatchmentArea> {
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
                let geometry = get_geometry_value(&feature.geometry.unwrap(), &area_key);
                let catchment_area = CatchmentArea {
                    meta: Default::default(),
                    area_key: area_key,
                    district_key: district_key,
                    district_name: district_name,
                    geometry: geometry,
                };
                println!("Catchment Area: {:?}", catchment_area);
                catchment_areas.push(catchment_area);
            }
        }
        _ => panic!("Failed to find feature collection in catchment area geojson"),
    };
    record_catchment_area_count(catchment_areas.len() as u64);
    return catchment_areas;
}

fn get_geometry_value(geometry: &Geometry, area_key: &String) -> GeometryValue {
    match &geometry.value {
        Value::Polygon(polygon) => {
            println!("Found polygon for : {}", area_key);
            return crate::models::catchment_area::GeometryValue::Polygon(polygon.to_vec());
        }
        Value::MultiPolygon(polygons) => {
            println!("Found multi polygon for : {}", area_key);
            return crate::models::catchment_area::GeometryValue::MultiPolygon(polygons.to_vec());
        }
        _ => panic!(
            "Failed to find polygon geometry in catchment area geojson: {}",
            area_key
        ),
    };
}
