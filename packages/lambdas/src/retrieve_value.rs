#[cfg(feature = "with-lambda")]
use lambda_http::{
    handler
};

#[cfg(not(feature = "with-lambda"))]
use std::io::{stdout, Write};

use serde_json::Value;

use lib::database::{get_db_client, retrieve_database_item};
use lib::logger::initialise_logger;
use lib::types::{Config, ConfigBuilder, CustomRetrieveValue, CustomValue, Error, Retrievable};

#[cfg(not(feature = "with-lambda"))]
use lib::error_and_panic;

#[cfg(not(feature = "with-lambda"))]
use log::error;

async fn retrieve_handler(config: Config, key: CustomRetrieveValue) -> Result<Value, Error> {
    let item_from_dynamo =
        retrieve_database_item(&config.table_name, &key, get_db_client(&config)?).await?;
    let retrieved_item = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap()).unwrap();
    Ok(retrieved_item.value().to_owned())
}

#[cfg(feature = "with-lambda")]
#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

    lambda_runtime::run(handler(|event, _| {
        let key = lambdas::extract_key_from_request(event);
        let input = CustomRetrieveValue { key };
        let config = ConfigBuilder::new()
            .table_name(lambdas::get_table_name())
            .build();

        retrieve_handler(config, input)
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
        error_and_panic!("You must pass a key input parameter as the first argument");
    }

    let input = CustomRetrieveValue {
        key: key_str.unwrap(),
    };

    let output = retrieve_handler(config, input).await?;
    stdout().write_all(output.to_string().as_bytes())?;
    Ok(())
}
