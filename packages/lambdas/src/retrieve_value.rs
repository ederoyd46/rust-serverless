use lib::types::{ConfigBuilder, CustomRetrieveValue};

use lambda_http::{service_fn, tower::BoxError};
use log::info;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    lambdas::initialise_logger()?;
    info!("Initialise Retrieve Value");

    let config = ConfigBuilder::new()
        .table_name(lambdas::get_table_name())
        .build()
        .await;

    lambda_http::run(service_fn(|event| {
        let key = lambdas::extract_key_from_request(event);
        lambdas::retrieve_handler(&config, CustomRetrieveValue { key })
    }))
    .await?;

    Ok(())
}
