use std::error::Error;
use std::env;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error, info};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, UpdateItemInput};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger;
use std::collections::HashMap;
// use cfg_if::{cfg_if};

// cfg_if! {
//     if #[cfg(test)] {
//         use self::thing::MockThing as Thing;
//     } else {
//         use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, UpdateItemInput};
//     }
// }


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
    let table_name = env::var("DATABASE").unwrap();

    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    let client = DynamoDbClient::new(Region::EuCentral1);

    let item = create_item(&e.first_name);
    let update_item = UpdateItemInput {
        key: item.clone(),
        table_name: table_name.to_string(),
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


#[cfg(test)]
mod tests {
    use mockall::{mock};
    use lambda_runtime::Context;
    use super::{CustomEvent, CustomOutput};
    use super::env::{set_var};

    mock! {
        DynamoDbClient {}
    }

    #[test]
    fn test_lambda_handler() {
        let mut _mock_dynamo_db_client = MockDynamoDbClient::new();
        set_var("DATABASE", "TEST");

        let expected_response = CustomOutput {
            message: "Hello First".to_string()
        };

        let lambda_context = Context {
            aws_request_id: "0123456789".to_string(),
            function_name: "test_function_name".to_string(),
            memory_limit_in_mb: 128,
            function_version: "$LATEST".to_string(),
            invoked_function_arn: "arn:aws:lambda".to_string(),
            xray_trace_id: Some("0987654321".to_string()),
            client_context: Option::default(),
            identity: Option::default(),
            log_stream_name: "logStreamName".to_string(),
            log_group_name: "logGroupName".to_string(),
            deadline: 0,
        };

        let lambda_request = CustomEvent {
            first_name: "First".to_string(),
        };

        // Check the result is ok
        let result = super::my_handler(lambda_request, lambda_context);
        assert_eq!(result.is_err(), false, "Error: {}", result.err().unwrap());

        // Confirm the expected values in result
        let value = result.ok().unwrap();
        assert_eq!(value.message, expected_response.message);
    }
}