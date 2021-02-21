use lambda_http::Request;
use lib::error_and_panic;
use log::{debug, error};
use std::env;

pub fn extract_key_from_request(event: Request) -> String {
    let path: Vec<&str> = event.uri().path().rsplit('/').collect();
    path.into_iter().next().unwrap().to_string()
}

pub fn get_table_name() -> String {
    match env::var("DATABASE") {
        Ok(table_name) => {
            debug!("Database table is {}", &table_name);
            table_name
        }
        Err(err) => error_and_panic!("Could not find TABLE_NAME from the environment", err),
    }
}