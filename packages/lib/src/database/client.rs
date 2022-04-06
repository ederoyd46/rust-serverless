use crate::types::{Config, Error};

use once_cell::sync::OnceCell;

static DB_CLIENT: OnceCell<aws_sdk_dynamodb::Client> = OnceCell::new();

pub async fn get_db_client(config: &Config) -> Result<&'static aws_sdk_dynamodb::Client, Error> {
    let aws_config = config.aws_config().await;
    Ok(DB_CLIENT.get_or_init(|| aws_sdk_dynamodb::Client::new(&aws_config)))
}
