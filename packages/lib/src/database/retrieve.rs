use log::{self, debug, error};
use rusoto_dynamodb::AttributeValue;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, GetItemOutput};
use std::collections::HashMap;

pub async fn retrieve_database_item(
    table_name: &str,
    key: &str,
    client: &DynamoDbClient,
) -> GetItemOutput {
    let get_item = GetItemInput {
        key: build_key_entry(key),
        table_name: table_name.to_string(),
        ..Default::default()
    };

    debug!("About to get from DynamoDB");
    let item_from_dynamo = match client.get_item(get_item).await {
        Ok(item) => item,
        Err(e) => {
            let error_message = format!("Error completing read to database: {}", e);
            error!("{}", &error_message);
            panic!(error_message);
        }
    };
    debug!("Retrieved from DynamoDB");

    item_from_dynamo
}

fn build_key_entry(key: &str) -> HashMap<String, AttributeValue> {
    let mut item = HashMap::new();
    item.insert(
        "PK".to_string(),
        AttributeValue {
            s: Some(key.to_string()),
            ..Default::default()
        },
    );

    item
}
