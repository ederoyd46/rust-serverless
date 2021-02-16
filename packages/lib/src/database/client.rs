use crate::types::Error;

use once_cell::sync::OnceCell;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

#[cfg(not(feature = "with-lambda"))]
const LOCAL_ENDPOINT: &str = "http://localhost:8000";

#[cfg(not(feature = "with-lambda"))]
const LOCAL_REGION: &str = "eu-central-1";

static DB_CLIENT: OnceCell<DynamoDbClient> = OnceCell::new();

pub fn get_db_client() -> Result<&'static DynamoDbClient, Error> {
    Ok(DB_CLIENT.get_or_init(|| DynamoDbClient::new(get_region())))
}

fn get_region() -> Region {
    #[cfg(feature = "with-lambda")]
    return Region::EuCentral1;

    #[cfg(not(feature = "with-lambda"))]
    Region::Custom {
        name: LOCAL_REGION.to_string(),
        endpoint: LOCAL_ENDPOINT.to_string(),
    }
}
