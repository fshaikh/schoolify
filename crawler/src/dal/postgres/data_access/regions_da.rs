use crate::models::error::Error;
use crate::models::region_config::RegionConfig;
use crate::services::id_service::parse_uuid;
use postgres::Transaction;

pub struct RegionsDA {}

impl RegionsDA {
    pub fn new() -> RegionsDA {
        return RegionsDA {};
    }

    pub fn delete(&self, transaction: &mut Transaction, id: &String) -> Result<bool, Error> {
        let result = transaction.execute("DELETE FROM regions WHERE id=$1", &[&parse_uuid(id)]);
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(Error {
                message: format!("Failed to delete region with id:{}. Error: {:?}", id, err),
            }),
        }
    }

    pub fn insert(
        &self,
        transaction: &mut Transaction,
        region: &RegionConfig,
    ) -> Result<bool, Error> {
        let result = transaction.execute("INSERT INTO regions(id, created_at,key,name,country,continent) VALUES ($1,now(),$2,$3,$4,$5)",
             &[&region.id,&region.key,&region.name,&region.country,&region.continent]);
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(Error {
                message: format!(
                    "Failed to insert region with id:{}. Error: {:?}",
                    region.id, err
                ),
            }),
        }
    }

}
