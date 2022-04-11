use lambda_http::{service_fn, Body, Request};

use serde_json::Value;

use lib::error_and_panic;
use lib::types::{ConfigBuilder, CustomValue, Error};

use log::{error, info};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambdas::initialise_logger()?;
    info!("Initialise Store Value");

    let config = ConfigBuilder::new()
        .table_name(lambdas::get_table_name())
        .build()
        .await;

    lambda_http::run(service_fn(|event: Request| {
        // info!("Event {:?}", event);
        let body = match event.body() {
            Body::Text(val) => val.as_ref(),
            Body::Binary(val) => std::str::from_utf8(val).unwrap(),
            Body::Empty => error_and_panic!("Invalid input, please use a string"),
        };
        let value: Value = match serde_json::from_str(body) {
            Ok(item) => item,
            Err(e) => error_and_panic!("Could not parse input to known type", e),
        };
        let key = lambdas::extract_key_from_request(event);
        let data = CustomValue { key, value };

        lambdas::store_handler(&config, data)
    }))
    .await?;

    Ok(())
}
