
use tokio_postgres::{Client,Connection,Transaction, Socket,Error, NoTls};
use crate::dal::objects::connection_details::ConnectionDetails;
use crate::dal::postgres::data_access::regions_async_da::RegionsDA;


pub struct PgRepository {
    pg_client: Client,
    pg_connection: Connection<Socket, tokio_postgres::tls::NoTlsStream>
}

impl PgRepository {
    pub async fn new(connection_details: &ConnectionDetails) -> Option<PgRepository> {
        let connect_result = connect_async(connection_details).await;
        println!("Created pg client");
        return match connect_result {
            Ok((client,connection)) => Some(PgRepository { pg_client: client, pg_connection: connection }),
            Err(err) => {
                println!("Unable to connect to database: {:?}", err);
                return None;
            }
        };
    }
    
    pub async fn delete_all(&mut self, region_id: &String) -> Result<bool, crate::models::error::Error> {
        let transaction_result = self.pg_client.transaction().await;
        let mut transaction = match transaction_result {
            Ok(tx) => Ok(mut tx),
            Err(_) => Err(crate::models::error::Error {
                message: "Unable to start a pg transaction".to_string(),
            }),
        };

        // SchoolsDA::new().delete(&mut transaction, region_id)?;
        RegionsDA::new().delete_async(&mut transaction, region_id)?;
        // CatchmentAreasDA::new().delete(&mut transaction, region_id)?;

        let tx_result = transaction.commit().await;

        match tx_result {
            _ => Ok(true),
            Err(err) => Err(crate::models::error::Error {
                message: "Error committing tx:".to_string(),
            }),
        }
    }

    // pub async fn start_transaction(&mut self) -> Result<Transaction, crate::models::error::Error> {
    //     let transaction_result = self.pg_client.transaction().await;
    //     match transaction_result {
    //         Ok(tx) => return Ok(tx),
    //         Err(_) => Err(crate::models::error::Error {
    //             message: "Unable to start a pg transaction".to_string(),
    //         }),
    //     }
    // }
}



async fn connect_async(
    connection_details: &ConnectionDetails,
) -> Result<(Client, Connection<Socket, tokio_postgres::tls::NoTlsStream>), Error> {
    let connection_string = format!(
        "host={} port={} user={} password={} dbname={}",
        connection_details.host,
        connection_details.port,
        connection_details.user_name,
        connection_details.password,
        connection_details.database
    );
    println!("{}", connection_string);
    tokio_postgres::connect(&connection_string, NoTls).await
}
