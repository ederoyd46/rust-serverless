use crate::log_and_throw;
use log::{self, debug, error};

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::PutItemError, output::PutItemOutput, types::SdkError};

use crate::types::Storable;

pub async fn store_database_item(
    table_name: &str,
    data: &impl Storable,
    client: &Client,
) -> Result<PutItemOutput, SdkError<PutItemError>> {
    // let put_item = PutItemInput {
    //     item: build_dynamo_db_map(data),
    //     table_name: table_name.to_string(),
    //     ..Default::default()
    // };

    debug!("About to update DynamoDB");
    let item_from_dynamo = match client
        .put_item()
        .table_name(table_name)
        .item("PK", data.get_pk())
        .item("value", data.to_dynamo_db())
        .send()
        .await
    {
        Ok(item) => item,
        Err(e) => log_and_throw!("Error completing write to database: {}", e),
    };
    debug!("Updated DynamoDB");

    Ok(item_from_dynamo)
}

// fn build_dynamo_db_map(data: &impl Storable) -> HashMap<String, AttributeValue> {
//     let mut item = HashMap::new();
//     item.insert(
//         "PK".to_string(),
//         AttributeValue {
//             s: Some(data.get_pk()),
//             ..Default::default()
//         },
//     );

//     item.extend(data.to_dynamo_db());

//     item
// }
