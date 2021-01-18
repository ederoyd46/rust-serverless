use crate::types::Error;

use log::debug;
use once_cell::sync::OnceCell;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

#[cfg(not(feature = "with-lambda"))]
const LOCAL_ENDPOINT: &str = "http://localhost:8000";

#[cfg(not(feature = "with-lambda"))]
const LOCAL_REGION: &str = "eu-central-1";

static DB_CLIENT: OnceCell<DynamoDbClient> = OnceCell::new();

pub fn get_db_client() -> Result<&'static DynamoDbClient, Error> {
    if DB_CLIENT.get().is_none() {
        debug!("About to create a client");
        let client = DynamoDbClient::new(get_region());
        assert_eq!(DB_CLIENT.set(client).is_err(), false); //TODO Do we really need this?
        debug!("Created a client");
    }
    Ok(DB_CLIENT.get().unwrap())
}

fn get_region() -> Region {
    #[cfg(feature = "with-lambda")]
    return Region::EuCentral1;

    #[cfg(not(feature = "with-lambda"))]
    Region::Custom {
        name: LOCAL_REGION.to_owned(),
        endpoint: LOCAL_ENDPOINT.to_owned(),
    }
}