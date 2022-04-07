use lib::types::{ConfigBuilder, CustomRetrieveValue, Error};

use lambda_http::service_fn;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambdas::initialise_logger()?;

    let config = ConfigBuilder::new()
        .table_name(lambdas::get_table_name())
        .build()
        .await;

    lambda_http::run(service_fn(|event| {
        let key = lambdas::extract_key_from_request(event);
        let key = CustomRetrieveValue { key };

        lambdas::retrieve_handler(&config, key)
    }))
    .await?;
    Ok(())
}
