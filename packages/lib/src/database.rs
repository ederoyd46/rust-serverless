use log::{self, debug, error};
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, UpdateItemInput, UpdateItemOutput,
};
use std::collections::HashMap;

use crate::types::CustomEvent;

pub async fn store_database_item(
    table_name: String,
    event: &CustomEvent,
    client: &DynamoDbClient,
) -> UpdateItemOutput {
    let item = create_item(&event.first_name);
    let update_item = UpdateItemInput {
        key: item.clone(),
        table_name: table_name.to_string(),
        ..Default::default()
    };

    debug!("About to update DynamoDB");
    let item_from_dynamo = match client.update_item(update_item).await {
        Ok(item) => item,
        Err(e) => {
            error!("Error completing write to database: {}", e);
            panic!("Error completing write to database: {}", e);
        }
    };
    debug!("Updated DynamoDB");

    item_from_dynamo
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
