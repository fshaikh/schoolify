use crate::models::error::Error;
use crate::models::region_config::RegionConfig;
use crate::services::id_service::parse_uuid;
use tokio_postgres::{Client,Connection,Transaction, Socket, NoTls};

pub struct RegionsDA {}

impl RegionsDA {
    pub fn new() -> RegionsDA {
        return RegionsDA {};
    }



    pub async fn delete_async(&self, transaction: &mut Transaction, id: &String) -> Result<bool, Error> {
        let result = transaction.execute("DELETE FROM regions WHERE id=$1", &[&parse_uuid(id)]).await;
        match result {
            Ok(_) => Ok(true),
            Err(err) => Err(Error {
                message: format!("Failed to delete region with id:{}. Error: {:?}", id, err),
            }),
        }
    }
}
