
use lib::database::{get_db_client, store_database_item};
use lib::logger::initialise_logger;
use lib::types::{CustomEvent, CustomOutput, Error};

#[cfg(feature = "with-lambda")]
use lambda::{lambda, Context};

use log::{debug, error, info};

// use serde_json::Value;

use std::env;

#[cfg(feature = "with-lambda")]
#[lambda]
#[tokio::main]
async fn main(event: CustomEvent, _: Context) -> Result<CustomOutput, Error> {
    initialise_logger()?;
    handler(event).await
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

    let input_str = std::env::args().nth(1);
    if input_str.is_none() {
        panic!("You must pass a JSON string input parameter as the first argument");
    }

    let input: CustomEvent = serde_json::from_str(&input_str.unwrap())?;
    let output = handler(input).await?;
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}

async fn handler(event: CustomEvent) -> Result<CustomOutput, Error> {
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    if event.first_name.is_empty() {
        error!("Empty first name in request");
        panic!("Empty first name");
    }

    let item_from_dynamo = store_database_item(&table_name, &event, get_db_client()?).await;

    info!("item: {:?}", item_from_dynamo);

    Ok(CustomOutput {
        message: format!("Hello, {}!", event.get_name()),
    })
}

// #[cfg(test)]
// mod tests {
//     use mockall::{mock};
//     use lambda_runtime::Context;
//     use super::{CustomEvent, CustomOutput};
//     use super::env::{set_var};

//     mock! {
//         DynamoDbClient {}
//     }

//     #[test]
//     fn test_lambda_handler() {
//         let mut _mock_dynamo_db_client = MockDynamoDbClient::new();
//         set_var("DATABASE", "TEST");

//         let expected_response = CustomOutput {
//             message: "Hello First".to_string()
//         };

//         let lambda_context = Context {
//             aws_request_id: "0123456789".to_string(),
//             function_name: "test_function_name".to_string(),
//             memory_limit_in_mb: 128,
//             function_version: "$LATEST".to_string(),
//             invoked_function_arn: "arn:aws:lambda".to_string(),
//             xray_trace_id: Some("0987654321".to_string()),
//             client_context: Option::default(),
//             identity: Option::default(),
//             log_stream_name: "logStreamName".to_string(),
//             log_group_name: "logGroupName".to_string(),
//             deadline: 0,
//         };

//         let lambda_request = CustomEvent {
//             first_name: "First".to_string(),
//         };

//         // Check the result is ok
//         let result = super::my_handler(lambda_request, lambda_context);
//         assert_eq!(result.is_err(), false, "Error: {}", result.err().unwrap());

//         // Confirm the expected values in result
//         let value = result.ok().unwrap();
//         assert_eq!(value.message, expected_response.message);
//     }
// }
