use lib::database::{get_db_client, store_database_item};
use lib::logger::initialise_logger;
use lib::types::{CustomOutput, Error, Storable, CustomValue};

#[cfg(feature = "with-lambda")]
use lambda::{lambda, Context};

use log::{debug, error, info};

use std::env;


#[cfg(feature = "with-lambda")]
#[lambda]
#[tokio::main]
async fn main(event: CustomValue, _: Context) -> Result<CustomOutput, Error> {
    handler(event).await
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let input_str = std::env::args().nth(1);

    if input_str.is_none() {
        panic!("You must pass a JSON string input parameter as the first argument");
    }

    let input: CustomValue = match serde_json::from_str(&input_str.unwrap()) {
        Ok(item) => item,
        Err(e) => {
            error!("Could not parse input to known type {}", e);
            panic!("Could not parse input to known type {}", e)
        }
    };

    let output = handler(input).await?;
    debug!("{}", serde_json::to_string(&output)?);
    Ok(())
}

async fn handler<T: Storable>(event: T) -> Result<CustomOutput, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    if !event.is_valid() {
        error!("No key specified");
        panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&table_name, &event, get_db_client()?).await;

    info!("item: {:?}", item_from_dynamo);

    Ok(CustomOutput {
        message: format!("Stored, {}!", event.get_pk()),
    })
}
