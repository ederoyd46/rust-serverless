use lambda_http::{service_fn, Body, Request};

use serde_json::Value;

use lib::error_and_panic;
use lib::logger::initialise_logger;
use lib::types::{ConfigBuilder, CustomValue, Error};

use log::error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

    lambda_http::run(service_fn(|event: Request| {
        let body = match event.body() {
            Body::Text(val) => val.as_ref(),
            _ => error_and_panic!("Invalid input, please use a string"), // Currently we only accept text
        };
        let value: Value = match serde_json::from_str(body) {
            Ok(item) => item,
            Err(e) => error_and_panic!("Could not parse input to known type", e),
        };
        let key = lambdas::extract_key_from_request(event);
        let data = CustomValue { key, value };
        let config = ConfigBuilder::new()
            .table_name(lambdas::get_table_name())
            .build();

        lambdas::store_handler(config, data)
    }))
    .await?;
    Ok(())
}
