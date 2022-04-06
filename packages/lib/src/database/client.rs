use crate::types::{Config, Error};

use once_cell::sync::OnceCell;

static DB_CLIENT: OnceCell<aws_sdk_dynamodb::Client> = OnceCell::new();

pub fn get_db_client(config: &Config) -> Result<&'static aws_sdk_dynamodb::Client, Error> {
    Ok(DB_CLIENT.get_or_init(|| aws_sdk_dynamodb::Client::new(&config.aws_sdk_config)))
}
