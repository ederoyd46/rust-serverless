use lib::logger::initialise_logger;
use lib::types::{ConfigBuilder, CustomRetrieveValue, Error};

use lambda_http::service_fn;

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialise_logger()?;

    lambda_http::run(service_fn(|event| {
        let key = lambdas::extract_key_from_request(event);
        let key = CustomRetrieveValue { key };
        let config = ConfigBuilder::new()
            .table_name(lambdas::get_table_name())
            .build();

        lambdas::retrieve_handler(config, key)
    }))
    .await?;
    Ok(())
}
