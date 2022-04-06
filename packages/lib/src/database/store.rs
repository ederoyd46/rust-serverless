use log::{self, debug};

use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::{error::PutItemError, output::PutItemOutput, types::SdkError};

use crate::types::Storable;

pub async fn store_database_item(
    table_name: &str,
    data: &impl Storable,
    client: &Client,
) -> Result<PutItemOutput, SdkError<PutItemError>> {
    debug!("About to update DynamoDB");
    client
        .put_item()
        .table_name(table_name)
        .item("PK", data.get_pk())
        .item("value", data.to_dynamo_db())
        .send()
        .await
}
