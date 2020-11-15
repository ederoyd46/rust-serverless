use lib::database::{get_db_client, store_database_item};
use lib::logger::initialise_logger;
use lib::types::{CustomEvent, CustomOutput, Error};

#[cfg(feature = "with-lambda")]
use lambda::{lambda, Context};

use log::{debug, error, info};

use std::env;

#[cfg(feature = "with-lambda")]
#[lambda]
#[tokio::main]
async fn main(event: CustomEvent, _: Context) -> Result<CustomOutput, Error> {
    handler(event).await
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let input_str = std::env::args().nth(1);
    if input_str.is_none() {
        panic!("You must pass a JSON string input parameter as the first argument");
    }

    let input: CustomEvent = serde_json::from_str(&input_str.unwrap())?;
    let output = handler(input).await?;
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

async fn handler(event: CustomEvent) -> Result<CustomOutput, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    if event.first_name.is_empty() {
        error!("Empty first name in request");
        panic!("Empty first name");
    }

    let item_from_dynamo = store_database_item(&table_name, &event, get_db_client()?).await;

    info!("item: {:?}", item_from_dynamo);

    Ok(CustomOutput {
        message: format!("Hello, {}!", event.get_name()),
    })
}
