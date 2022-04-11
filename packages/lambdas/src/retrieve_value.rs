use lib::types::{ConfigBuilder, CustomRetrieveValue, Error};

use lambda_http::service_fn;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
