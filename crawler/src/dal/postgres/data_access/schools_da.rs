use crate::models::error::Error;
use crate::models::school::School;
use crate::services::id_service::{get_unique_id, parse_uuid};
use postgres::Transaction;

const TABLE: &str = "schools";

pub struct SchoolsDA {}

impl SchoolsDA {
    pub fn new() -> SchoolsDA {
        return SchoolsDA {};
    }

    pub fn delete(&self, transaction: &mut Transaction, id: &String) -> Result<bool, Error> {
        let result =
            transaction.execute("DELETE FROM schools WHERE region_id=$1", &[&parse_uuid(id)]);
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(Error {
                message: format!("Failed to delete schools with id:{}. Error: {:?}", id, err),
            }),
        }
    }

    pub fn insert(&self, transaction: &mut Transaction, schools: &Vec<School>) {
        schools.iter().for_each(|school| {
            let id_school = get_unique_id();
            let location = school.get_location_vector();
            let catchment_area_id = self.get_catchmentarea_id(&school);
            let school_result = transaction.execute(
                "INSERT INTO schools (id, created_at,school_id,school_name,location,region_id,catchmentarea_id)
                 VALUES ($1, now(), $2, $3, ST_SetSRID(ST_MakePoint($4, $5),4326), $6, $7)",
                &[&id_school,&school.school_id, &school.name, &location[0], &location[1], &parse_uuid(&school.region_id), &catchment_area_id],
            );
            match school_result {
                Ok(r) => println!("created school: {}",id_school),
                Err(err) => println!("{:?}", err),
            }
        });
    }

    fn get_catchmentarea_id(&self, school: &School) -> String {
        match &school.catchmentarea_id {
            Some(id) => id.to_string(),
            None => "NULL".to_string(),
        }
    }
}
