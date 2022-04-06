use crate::types::CustomRetrieveValue;
use aws_sdk_dynamodb::{
    error::GetItemError, model::AttributeValue, output::GetItemOutput, types::SdkError,
};

use log::{self, debug};

use aws_sdk_dynamodb::Client;

pub async fn retrieve_database_item(
    table_name: &str,
    retrieve_value: &CustomRetrieveValue,
    client: &Client,
) -> Result<GetItemOutput, SdkError<GetItemError>> {
    debug!("About to get from DynamoDB {:?}", &retrieve_value.key);
    client
        .get_item()
        .table_name(table_name)
        .key("PK", AttributeValue::S(retrieve_value.key.to_string()))
        .send()
        .await
}
