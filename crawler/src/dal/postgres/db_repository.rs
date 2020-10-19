use postgres::{Client, Error, NoTls, Transaction};

use crate::dal::objects::connection_details::ConnectionDetails;
use crate::dal::postgres::data_access::catchmentareas_da::CatchmentAreasDA;
use crate::dal::postgres::data_access::regions_da::RegionsDA;
use crate::dal::postgres::data_access::schools_da::SchoolsDA;
use crate::models::catchment_area::CatchmentArea;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::region_config::RegionConfig;

pub struct PgRepository {
    pg_client: Client,
}

impl PgRepository {
    pub fn new(connection_details: &ConnectionDetails) -> Option<PgRepository> {
        let client_result = connect(connection_details);
        println!("Created pg client");
        return match client_result {
            Ok(client) => Some(PgRepository { pg_client: client }),
            Err(err) => {
                println!("Unable to connect to database: {:?}", err);
                return None;
            }
        };
    }
    pub fn delete_all(&mut self, region_id: &String) -> Result<bool, crate::models::error::Error> {
        let mut transaction = self.start_transaction()?;

        SchoolsDA::new().delete(&mut transaction, region_id)?;
        RegionsDA::new().delete(&mut transaction, region_id)?;
        CatchmentAreasDA::new().delete(&mut transaction, region_id)?;

        let tx_result = transaction.commit();

        match tx_result {
            _ => Ok(true),
            Err(err) => Err(crate::models::error::Error {
                message: "Error committing tx:".to_string(),
            }),
        }
    }

    pub fn insert_all(
        &mut self,
        crawler_request: &CrawlerRequest,
        crawler_response: &CrawlerResponse,
    ) -> Result<bool, crate::models::error::Error> {
        let mut transaction = self.start_transaction()?;
        RegionsDA::new().insert(&mut transaction, &crawler_request.region)?;
        CatchmentAreasDA::new().insert(&mut transaction, &crawler_response.catchmentareas);
        SchoolsDA::new().insert(&mut transaction, &crawler_response.schools);
        let tx_result = transaction.commit();

        match tx_result {
            _ => Ok(true),
            Err(err) => Err(crate::models::error::Error {
                message: "Error committing tx:".to_string(),
            }),
        }
    }

    pub fn start_transaction(&mut self) -> Result<Transaction, crate::models::error::Error> {
        let transaction_result = self.pg_client.transaction();
        match transaction_result {
            Ok(tx) => return Ok(tx),
            Err(_) => Err(crate::models::error::Error {
                message: "Unable to start a pg transaction".to_string(),
            }),
        }
    }
}

fn connect(connection_details: &ConnectionDetails) -> Result<Client, Error> {
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        connection_details.host,
        connection_details.port,
        connection_details.user_name,
        connection_details.password,
        connection_details.database
    );
    println!("{}", connection_string);
    return Client::connect(&connection_string, NoTls);
}
