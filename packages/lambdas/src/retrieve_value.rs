use lib::database::{get_db_client, retrieve_database_item};
use lib::logger::initialise_logger;
use lib::types::{CustomOutput, CustomValue, Error};

#[cfg(feature = "with-lambda")]
use lambda::{lambda, Context};

use log::debug;

use std::env;

#[cfg(feature = "with-lambda")]
#[lambda]
#[tokio::main]
async fn main(event: String, _: Context) -> Result<CustomOutput, Error> {
    handler(&event).await
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let input_str = std::env::args().nth(1);

    if input_str.is_none() {
        panic!("You must pass a key input parameter as the first argument");
    }

    let input = input_str.unwrap();

    let output = handler(&input).await?;
    debug!("{:?}", output);
    Ok(())
}

async fn handler(key: &str) -> Result<CustomOutput, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    let item_from_dynamo = retrieve_database_item(&table_name, key, get_db_client()?).await;
    debug!("{:?}", &item_from_dynamo);
    let value = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap());

    Ok(CustomOutput {
        message: format!("Retrieved, {:?}!", value),
    })
}
