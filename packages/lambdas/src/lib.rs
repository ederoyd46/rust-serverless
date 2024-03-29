use lambda_http::{tower::BoxError, Request};
use lib::database::{retrieve_database_item, store_database_item};
use lib::error_and_panic;
use lib::types::{Config, CustomRetrieveValue, CustomValue, Retrievable, Storable};
use log::{debug, error, info, LevelFilter, SetLoggerError};
use serde_json::Value;
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

pub async fn retrieve_handler(config: &Config, key: CustomRetrieveValue) -> Result<Value, BoxError> {
    let item_from_dynamo =
        retrieve_database_item(&config.table_name, &key, &config.dynamodb).await?;
    let retrieved_item = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap()).unwrap();
    Ok(retrieved_item.value().to_owned())
}

pub async fn store_handler<T: Storable>(config: &Config, data: T) -> Result<String, BoxError> {
    if !data.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&config.table_name, &data, &config.dynamodb).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(format!("Stored, {}!", data.get_pk().as_s().unwrap()))
}

pub fn initialise_logger() -> Result<(), SetLoggerError> {
    env_logger::builder()
        .filter(Some("store_value"), LevelFilter::Debug)
        .filter(Some("retrieve_value"), LevelFilter::Debug)
        .filter(Some("lib::database"), LevelFilter::Debug)
        .filter_level(LevelFilter::Info)
        .init();
    Ok(())
}
