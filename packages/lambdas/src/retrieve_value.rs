#[cfg(feature = "with-lambda")]
use lambda::{lambda, Context};

use serde_json::Value;

#[cfg(not(feature = "with-lambda"))]
use lib::error_and_panic;

use lib::database::{get_db_client, retrieve_database_item};
use lib::logger::initialise_logger;
use lib::types::{CustomRetrieveValue, CustomValue, Error, Retrievable};

use log::debug;

#[cfg(not(feature = "with-lambda"))]
use log::error;

use std::env;

#[cfg(feature = "with-lambda")]
#[lambda]
#[tokio::main]
async fn main(event: CustomRetrieveValue, _: Context) -> Result<Value, Error> {
    handler(&event).await
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let input_str = std::env::args().nth(1);

    if input_str.is_none() {
        error_and_panic!("You must pass a key input parameter as the first argument");
    }

    let input: CustomRetrieveValue = serde_json::from_str(&input_str.unwrap())?;
    let output = handler(&input).await?;
    debug!("{}", output.to_string());

    Ok(())
}

async fn handler(key: &CustomRetrieveValue) -> Result<Value, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    let item_from_dynamo = retrieve_database_item(&table_name, key, get_db_client()?).await;
    let retrieved_item = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap()).unwrap();
    debug!("Retrieved Item {:?}", retrieved_item);
    Ok(retrieved_item.value().clone())
}
