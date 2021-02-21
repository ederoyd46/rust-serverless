use crate::types::{Config, Error};

use once_cell::sync::OnceCell;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::str::FromStr;

static DB_CLIENT: OnceCell<DynamoDbClient> = OnceCell::new();

pub fn get_db_client(config: &Config) -> Result<&'static DynamoDbClient, Error> {
    let region = get_region(config)?;
    Ok(DB_CLIENT.get_or_init(|| DynamoDbClient::new(region)))
}

fn get_region(config: &Config) -> Result<Region, Error> {
    let region = match &config.aws_sdk_endpoint {
        Some(endpoint) => Region::Custom {
            name: config.aws_sdk_region.to_string(),
            endpoint: endpoint.to_string(),
        },
        None => Region::from_str(&config.aws_sdk_region)?,
    };

    Ok(region)
}
