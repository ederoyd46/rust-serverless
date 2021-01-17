#[cfg(feature = "with-lambda")]
use lambda_http::{
    lambda::{lambda, Context},
    IntoResponse, Request,
};

#[cfg(not(feature = "with-lambda"))]
use std::io::{stdout, Write};

use serde_json::Value;

use lib::error_and_panic;

use lib::database::{get_db_client, retrieve_database_item};
use lib::logger::initialise_logger;
use lib::types::{CustomRetrieveValue, CustomValue, Error, Retrievable};

use log::{debug, error};
use std::env;

#[cfg(feature = "with-lambda")]
#[lambda(http)]
#[tokio::main]
async fn main(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    debug!("Retrieve: {:?}", event);
    let key = lambdas::extract_key_from_request(event);

    let input = CustomRetrieveValue { key };

    match handler(input).await {
        Ok(val) => Ok(val),
        Err(e) => error_and_panic!("Could not retrieve data", e),
    }
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_str = std::env::args().nth(1);

    if key_str.is_none() {
        error_and_panic!("You must pass a key input parameter as the first argument");
    }

    let input = CustomRetrieveValue {
        key: key_str.unwrap(),
    };

    let output = handler(input).await?;
    stdout().write_all(output.to_string().as_bytes())?;
    Ok(())
}

async fn handler(key: CustomRetrieveValue) -> Result<Value, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE")?;
    debug!("Database table is {}", table_name);

    let item_from_dynamo = retrieve_database_item(&table_name, &key, get_db_client()?).await?;
    let retrieved_item = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap()).unwrap();
    Ok(retrieved_item.value().clone()) //TODO Can I get rid of this clone?
}
