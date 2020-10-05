use std::error::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error, info};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, UpdateItemInput};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;
use std::collections::HashMap;

#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);
    Ok(())
}

fn my_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    let client = DynamoDbClient::new(Region::EuCentral1);

    let item = create_item(&e.first_name);
    let update_item = UpdateItemInput {
        key: item.clone(),
        table_name: "rust_serverless_store_dev".to_string(),
        ..Default::default()
    };

    let item_from_dynamo = match client.update_item(update_item).sync() {
        Ok(item) => item,
        Err(e) => panic!("Error completing futures: {}", e),
    };

    info!("item: {:?}", item_from_dynamo);

    Ok(CustomOutput {
        message: format!("Hello, {}!", e.first_name),
    })
}

fn create_item(first_name: &String) -> HashMap<String, AttributeValue> {
    let item_key = "firstName";
    let mut item = HashMap::new();
    item.insert(
        item_key.to_string(),
        AttributeValue {
            s: Some(first_name.to_string()),
            ..Default::default()
        },
    );
    item
}
