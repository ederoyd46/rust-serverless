use lambda_http::Request;
use lib::database::{get_db_client, retrieve_database_item, store_database_item};
use lib::error_and_panic;
use lib::types::{Config, CustomRetrieveValue, CustomValue, Error, Retrievable, Storable};
use log::{debug, error, info};
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

pub async fn retrieve_handler(config: Config, key: CustomRetrieveValue) -> Result<Value, Error> {
    let item_from_dynamo =
        retrieve_database_item(&config.table_name, &key, get_db_client(&config)?).await?;
    let retrieved_item = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap()).unwrap();
    Ok(retrieved_item.value().to_owned())
}

pub async fn store_handler<T: Storable>(config: Config, data: T) -> Result<String, Error> {
    if !data.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo =
        store_database_item(&config.table_name, &data, get_db_client(&config)?).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(format!("Stored, {}!", data.get_pk()))
}
