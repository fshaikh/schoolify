use crate::dal::postgres::db_repository::PgRepository;
use crate::models::crawler_request::CrawlerRequest;
use crate::models::crawler_response::CrawlerResponse;
use crate::models::error::Error;

pub fn handle_db_operations(
    crawler_request: &CrawlerRequest,
    crawler_response: &CrawlerResponse,
) -> Result<bool, Error> {
    // orchestrate all database operations
    // Get the db repository. Pg for now
    let pg_repository_option = PgRepository::new(&crawler_request.database_config);
    match pg_repository_option {
        Some(mut pg_repository) => {
            let delete_result = delete_all(crawler_request, &mut pg_repository)?;
            if (!delete_result) {
                return Err(Error {
                    message: "Unable to delete".to_string(),
                });
            }
            insert_all(crawler_request, crawler_response, &mut pg_repository)
        }
        None => {
            return Err(Error {
                message: "".to_string(),
            });
        }
    }
}

fn delete_all(
    crawler_request: &CrawlerRequest,
    pg_repository: &mut PgRepository,
) -> Result<bool, Error> {
    let result = pg_repository.delete_all(&crawler_request.region.id);
    match result {
        Ok(r) => {
            println!("Successfully deleted");
            return Ok(r);
        }
        Err(err) => {
            println!("{:?}", err);
            return Err(err);
        }
    }
}

fn insert_all(
    crawler_request: &CrawlerRequest,
    crawler_response: &CrawlerResponse,
    pg_repository: &mut PgRepository,
) -> Result<bool, Error> {
    pg_repository.insert_all(crawler_request, crawler_response)
}
