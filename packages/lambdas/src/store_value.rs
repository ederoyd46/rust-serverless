use lambda_http::{service_fn, Body, Request};

use serde_json::Value;

use lib::database::{get_db_client, store_database_item};
use lib::error_and_panic;
use lib::logger::initialise_logger;
use lib::types::{Config, ConfigBuilder, CustomValue, Error, Storable};

use log::{error, info};

async fn handle_store<T: Storable>(config: Config, event: T) -> Result<String, Error> {
    if !event.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo =
        store_database_item(&config.table_name, &event, get_db_client(&config)?).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(format!("Stored, {}!", event.get_pk()))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

    lambda_http::run(service_fn(|event: Request| {
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
