use lambda_http::handler;

use serde_json::Value;

use lib::database::{get_db_client, retrieve_database_item};
use lib::logger::initialise_logger;
use lib::types::{Config, ConfigBuilder, CustomRetrieveValue, CustomValue, Error, Retrievable};

async fn retrieve_handler(config: Config, key: CustomRetrieveValue) -> Result<Value, Error> {
    let item_from_dynamo =
        retrieve_database_item(&config.table_name, &key, get_db_client(&config)?).await?;
    let retrieved_item = CustomValue::from_dynamo_db(item_from_dynamo.item.unwrap()).unwrap();
    Ok(retrieved_item.value().to_owned())
}

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
