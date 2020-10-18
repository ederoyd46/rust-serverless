use lib::Error;

use log::debug;
use once_cell::sync::OnceCell;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

static DB_CLIENT: OnceCell<DynamoDbClient> = OnceCell::new();

pub fn get_db_client() -> Result<&'static DynamoDbClient, Error> {
    if DB_CLIENT.get().is_none() {
        debug!("About to create a client");
        let client = DynamoDbClient::new(Region::EuCentral1);
        assert_eq!(DB_CLIENT.set(client).is_err(), false);
        debug!("Created a client");
    }
    Ok(DB_CLIENT.get().unwrap())
}
