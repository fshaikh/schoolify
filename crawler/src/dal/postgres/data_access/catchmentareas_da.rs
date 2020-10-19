use crate::dal::postgres::type_adaptors::polygon::tosql_polygon_geometry;
use crate::models::catchment_area::CatchmentArea;
use crate::models::error::Error;
use crate::services::id_service::{get_unique_id, parse_uuid};
use postgres::Transaction;

const TABLE: &str = "catchmentareas";

pub struct CatchmentAreasDA {}

impl CatchmentAreasDA {
    pub fn new() -> CatchmentAreasDA {
        return CatchmentAreasDA {};
    }

    pub fn delete(&self, transaction: &mut Transaction, id: &String) -> Result<bool, Error> {
        let result = transaction.execute(
            "DELETE FROM catchmentareas WHERE region_id=$1",
            &[&parse_uuid(id)],
        );
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(Error {
                message: format!(
                    "Failed to delete catchment areas with id:{}. Error: {:?}",
                    id, err
                ),
            }),
        }
    }

    pub fn insert(&self, transaction: &mut Transaction, catchment_areas: &Vec<CatchmentArea>) {
        catchment_areas.iter().for_each(|catchment_area| {
            let id_ca = get_unique_id();
            let geometry_value = tosql_polygon_geometry(&catchment_area.geometry);

            let ca_result = transaction.execute(
                            "INSERT INTO catchmentareas (id, created_at,area_key,district_key,district_name,polygon,region_id) VALUES ($1, now(), $2, $3,$4, ST_GeomFromText($5,4326), $6)",
                            &[&id_ca, &catchment_area.area_key, &catchment_area.district_key, &catchment_area.district_name,&geometry_value, &parse_uuid(&catchment_area.region_id)],
                        );
        });
    }
}
