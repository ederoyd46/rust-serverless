use log::{self, debug, error};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, PutItemOutput};
// use serde_json::Value;

use crate::types::{CustomEvent, Storable};

pub async fn store_database_item(
    table_name: &str,
    event: &CustomEvent,
    client: &DynamoDbClient,
) -> PutItemOutput {
    let put_item = PutItemInput {
        item: event.to_dynamo_db(),
        table_name: table_name.to_string(),
        ..Default::default()
    };

    debug!("About to update DynamoDB");
    let item_from_dynamo = match client.put_item(put_item).await {
        Ok(item) => item,
        Err(e) => {
            error!("Error completing write to database: {}", e);
            panic!("Error completing write to database: {}", e);
        }
    };
    debug!("Updated DynamoDB");

    item_from_dynamo
}