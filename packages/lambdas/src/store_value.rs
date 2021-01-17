#[cfg(feature = "with-lambda")]
use lambda_http::{
    lambda::{lambda, Context},
    Body, IntoResponse, Request,
};

use serde_json::Value;

use lib::database::{get_db_client, store_database_item};
use lib::error_and_panic;
use lib::logger::initialise_logger;
use lib::types::{CustomOutput, CustomValue, Error, Storable};

use log::{debug, error, info};

#[cfg(not(feature = "with-lambda"))]
use std::fs::read_to_string;

use std::env;

#[cfg(feature = "with-lambda")]
#[lambda(http)]
#[tokio::main]
async fn main(event: Request, _context: Context) -> Result<impl IntoResponse, Error> {
    let body = match event.body() {
        Body::Text(val) => val.as_ref(),
        _ => error_and_panic!("Invalid input, please use a string"), // Currently we only accept text
    };

    let value: Value = match serde_json::from_str(body) {
        Ok(item) => item,
        Err(e) => error_and_panic!("Could not parse input to known type", e),
    };

    let key = lambdas::extract_key_from_request(event);
    let input = CustomValue { key, value };

    match handler(input).await {
        Ok(val) => Ok(val.body),
        Err(e) => error_and_panic!("Could not store data", e),
    }
}

#[cfg(not(feature = "with-lambda"))]
#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_str = std::env::args().nth(1);
    if key_str.is_none() {
        panic!("You must pass a Key input parameter as the first argument");
    }

    let path_str = std::env::args().nth(2);
    if path_str.is_none() {
        panic!("You must pass a Path parameter as the second argument");
    }

    let value_str = read_to_string(path_str.unwrap());

    let value: Value = match serde_json::from_str(&value_str.unwrap()) {
        Ok(item) => item,
        Err(e) => error_and_panic!("Could not parse input to known type", e),
    };

    let input = CustomValue {
        key: key_str.unwrap(),
        value,
    };

    let output = handler(input).await?;
    debug!("{}", serde_json::to_string(&output)?);
    Ok(())
}

async fn handler<T: Storable>(event: T) -> Result<CustomOutput, Error> {
    initialise_logger()?;
    let table_name = env::var("DATABASE").unwrap();
    debug!("Database table is {}", table_name);

    if !event.is_valid() {
        error_and_panic!("No key specified");
    }

    let item_from_dynamo = store_database_item(&table_name, &event, get_db_client()?).await?;

    info!("item: {:?}", item_from_dynamo);

    Ok(CustomOutput {
        body: format!("Stored, {}!", event.get_pk()),
        status: 200,
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
