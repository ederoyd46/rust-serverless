#[cfg(feature = "with-lambda")]
use lambda_http::{
    handler,
    lambda::{self},
    Body, Request
};

use serde_json::Value;

use lib::database::{get_db_client, store_database_item};
use lib::error_and_panic;
use lib::logger::initialise_logger;
use lib::types::{CustomValue, Error, Storable};

use log::{debug, error, info};

#[cfg(not(feature = "with-lambda"))]
use std::fs::read_to_string;

use std::env;

async fn handle_store<T: Storable>(event: T) -> Result<String, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    if !event.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&table_name, &event, get_db_client()?).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(format!("Stored, {}!", event.get_pk()))
}

#[cfg(feature = "with-lambda")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(|event: Request, _| {
        let body = match event.body() {
            Body::Text(val) => val.as_ref(),
            _ => error_and_panic!("Invalid input, please use a string"), // Currently we only accept text
        };
        let value: Value = match serde_json::from_str(body) {
            Ok(item) => item,
            Err(e) => error_and_panic!("Could not parse input to known type", e),
        };
        let key = lambdas::extract_key_from_request(event);
        let input = CustomValue { key, value };
        handle_store(input)
    })).await?;
    Ok(())
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_str = std::env::args().nth(1);
    if key_str.is_none() {
        panic!("You must pass a Key input parameter as the first argument");
    }

    let path_str = std::env::args().nth(2);
    if path_str.is_none() {
        panic!("You must pass a Path parameter as the second argument");
    }

    let value_str = read_to_string(path_str.unwrap());

    let value: Value = match serde_json::from_str(&value_str.unwrap()) {
        Ok(item) => item,
        Err(e) => error_and_panic!("Could not parse input to known type", e),
    };

    let input = CustomValue {
        key: key_str.unwrap(),
        value,
    };

    let output = handle_store(input).await?;
    debug!("{}", output);
    Ok(())
}
