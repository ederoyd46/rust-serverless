use lambda_http::Request;
use lib::error_and_panic;
use log::{debug, error};
use std::env;

pub fn extract_key_from_request(event: Request) -> String {
    match event.uri().path().rsplit_once('/') {
        Some((_prefix, path_name)) => path_name.to_string(),
        None => error_and_panic!("Could not find Path from the environment"),
    }
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
