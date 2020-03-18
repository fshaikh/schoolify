use crate::crawlers::berlin::models::details_result::DetailsResult;
use crate::crawlers::berlin::models::school_results::{SchoolResult, SchoolResults};
use crate::models::catchment_area::CatchmentArea;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::error::Error;
use crate::models::location::Location;
use crate::models::mapping::Mapping;
use crate::models::object_base::ObjectBase;
use crate::models::school::{FundingType, School, SchoolType};
use crate::services::geo_service::point_within_polygon;

pub fn map(
    school_results: &SchoolResults,
    catchment_areas: Vec<CatchmentArea>,
) -> Result<CrawlerResponse, Error> {
    let mut schools: Vec<School> = Vec::new();
    let mut mapping = Mapping::new();
    school_results.results.iter().for_each(|school_result| {
        let school = map_school(school_result, &catchment_areas);
        println!("Mapped School: {:?}", school);
        match school.get_catchmentarea_id() {
            Some(id) => mapping.add_school(id.clone(), school.get_id().clone()),
            None => {}
        }
        schools.push(school);
    });
    return Ok(CrawlerResponse {
        schools: schools,
        catchmentareas: catchment_areas,
        mapping: mapping,
    });
}

fn map_school(school_result: &SchoolResult, catchment_areas: &Vec<CatchmentArea>) -> School {
    let meta: ObjectBase = Default::default();
    School {
        meta: meta,
        address: school_result.details_result.get_address(),
        catchmentarea_id: map_catchment_area_id(school_result, catchment_areas),
        contact_persons: vec![school_result.details_result.get_primary_contact()],
        district: school_result.root_result.district.clone(),
        email: school_result.details_result.get_email(),
        fees: "".to_string(), // TODO
        name: school_result.root_result.school_name.clone(),
        url: school_result.details_result.get_url(),
        school_id: school_result.root_result.id.clone(),
        funding_type: FundingType::Private, // TODO
        is_bilingual: map_is_bilingual(&school_result.details_result),
        languages: school_result.details_result.get_languages(),
        location: map_location(&school_result.details_result.location),
        primary_phone_number: school_result.details_result.get_primary_contact(),
        school_type: SchoolType::Primary, // TODO
    }
}

fn map_is_bilingual(details_result: &DetailsResult) -> bool {
    match &details_result.languages {
        None => false,
        Some(languages) => {
            if languages.iter().count() > 1 {
                return true;
            }
            return false;
        }
    }
}

fn map_location(location: &Option<Location>) -> Option<Location> {
    location.clone()
}

fn map_catchment_area_id(
    school_result: &SchoolResult,
    catchment_areas: &Vec<CatchmentArea>,
) -> Option<String> {
    get_catchmentarea_for_school(&school_result.details_result, catchment_areas)
}

pub fn get_catchmentarea_for_school(
    school_details_result: &DetailsResult,
    catchment_areas: &Vec<CatchmentArea>,
) -> Option<String> {
    catchment_areas
        .iter()
        .find(|catchment_area| match &school_details_result.location {
            None => false,
            Some(location) => point_within_polygon(&location, &catchment_area.geometry),
        })
        .map(|catchment_area| catchment_area.meta.id.clone())
}

//#region Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::crawlers::berlin::models::details_result::DetailsResult;
    use crate::crawlers::berlin::models::root_result::RootResult;
    use crate::crawlers::berlin::models::school_results::SchoolResult;
    use crate::models::catchment_area::CatchmentArea;
    use crate::models::catchment_area::GeometryValue;
    use crate::models::location::Location;
    use crate::models::object_base::ObjectBase;

    #[test]
    fn it_should_get_catchmentarea_for_school_with_location() {
        let details_result: DetailsResult = Default::default();
        let positions = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let catchment_area = CatchmentArea {
            meta: ObjectBase {
                id: "1".to_string(),
                ..Default::default()
            },
            geometry: GeometryValue::Polygon(vec![positions]),
            ..Default::default()
        };

        let result = get_catchmentarea_for_school(&details_result, &vec![catchment_area]);
        assert_eq!(result, Some("1".to_string()));
    }

    #[test]
    fn it_should_get_none_catchmentarea_for_school_without_location() {
        let details_result_without_location = DetailsResult {
            location: None,
            ..Default::default()
        };
        let positions = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let catchment_area = CatchmentArea {
            geometry: GeometryValue::Polygon(vec![positions]),
            ..Default::default()
        };

        let result =
            get_catchmentarea_for_school(&details_result_without_location, &vec![catchment_area]);
        assert_eq!(result, None);
    }

    #[test]
    fn it_should_get_nonecatchmentarea_for_school_outside_location() {
        let details_result = DetailsResult {
            location: Some(Location {
                longitude: 23.426008,
                latitude: 52.536780,
            }),
            ..Default::default()
        };
        let positions = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let catchment_area = CatchmentArea {
            geometry: GeometryValue::Polygon(vec![positions]),
            ..Default::default()
        };

        let result = get_catchmentarea_for_school(&details_result, &vec![catchment_area]);
        assert_eq!(result, None);
    }

    #[test]
    fn it_should_map_school_correctly() {
        let root_result = RootResult {
            id: "1234".to_string(),
            details_url: "https://url".to_string(),
            school_name: "an der marie".to_string(),
            sub_district: "prenzaluer berg".to_string(),
            district: "pankow".to_string(),
            school_type: "grundschule".to_string(),
        };

        let details_result = DetailsResult {
            address: Some("address".to_string()),
            tel: Some("1234-567-890".to_string()),
            fax: Some("1234-567-891".to_string()),
            email: Some("secretary@andermarie.com".to_string()),
            url: Some("andermarie.de".to_string()),
            primary_contact: Some("Frau Katja".to_string()),
            languages: None,
            location: Some(Location {
                longitude: 13.426008,
                latitude: 52.536780,
            }),
        };

        let school_result = SchoolResult {
            root_result: root_result,
            details_result: details_result,
        };

        let positions = vec![
            vec![13.425781731803541, 52.534321477991661],
            vec![13.422081412762472, 52.535570557220687],
            vec![13.423027639945467, 52.537143026809822],
            vec![13.424210560058448, 52.53917959650277],
            vec![13.433057967203794, 52.53617852176172],
            vec![13.431807020385165, 52.535120557971489],
            vec![13.429434731727476, 52.533061419699571],
            vec![13.425781731803541, 52.534321477991661],
        ];
        let catchment_area = CatchmentArea {
            meta: ObjectBase {
                id: "1".to_string(),
                ..Default::default()
            },
            geometry: GeometryValue::Polygon(vec![positions]),
            ..Default::default()
        };

        let school = map_school(&school_result, &vec![catchment_area]);
        assert_eq!(school.name, "an der marie".to_string());
        match school.catchmentarea_id {
            None => assert_eq!(true, false),
            Some(id) => assert_eq!(id, "1".to_string()),
        }
    }
}

//#endregion Tests
