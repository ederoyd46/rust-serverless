#[cfg(feature = "with-lambda")]
use lambda_http::{
    handler,
    lambda::{self},
    Body, Request,
};

use serde_json::Value;

use lib::database::{get_db_client, store_database_item};
use lib::error_and_panic;
use lib::logger::initialise_logger;
use lib::types::{Config, ConfigBuilder, CustomValue, Error, Storable};

use log::{error, info};

#[cfg(not(feature = "with-lambda"))]
use log::{debug};


#[cfg(not(feature = "with-lambda"))]
use lib::log_and_exit;

#[cfg(not(feature = "with-lambda"))]
use std::fs::read_to_string;

async fn handle_store<T: Storable>(config: Config, event: T) -> Result<String, Error> {
    if !event.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo =
        store_database_item(&config.table_name, &event, get_db_client(&config)?).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(format!("Stored, {}!", event.get_pk()))
}

#[cfg(feature = "with-lambda")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

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
        let config = ConfigBuilder::new()
            .table_name(lambdas::get_table_name())
            .build();

        handle_store(config, input)
    }))
    .await?;
    Ok(())
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

    let config = ConfigBuilder::new()
        .table_name(lambdas::get_table_name())
        .aws_sdk_endpoint(Some("http://localhost:8000".to_string()))
        .build();

    let key_str = std::env::args().nth(1);
    if key_str.is_none() {
        log_and_exit!(
            "You must pass a Key input parameter as the first argument",
            1
        );
    }

    let path_str = std::env::args().nth(2);
    if path_str.is_none() {
        log_and_exit!("You must pass a Path parameter as the second argument", 2);
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

    let output = handle_store(config, input).await?;
    debug!("{}", output);
    Ok(())
}
