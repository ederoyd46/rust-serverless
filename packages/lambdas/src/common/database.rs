use lib::Error;

use log::debug;
use once_cell::sync::OnceCell;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;

static DB_CLIENT: OnceCell<DynamoDbClient> = OnceCell::new();

pub fn get_db_client() -> Result<&'static DynamoDbClient, Error> {

    #[cfg(feature = "with-lambda")]
    let region = Region::EuCentral1;

    #[cfg(not(feature = "with-lambda"))]
    let region = Region::Custom {
        name: "eu-central-1".to_owned(),
        endpoint: "http://localhost:8000".to_owned(),
    };

    if DB_CLIENT.get().is_none() {
        debug!("About to create a client");
        let client = DynamoDbClient::new(region);
        assert_eq!(DB_CLIENT.set(client).is_err(), false);
        debug!("Created a client");
    }
    Ok(DB_CLIENT.get().unwrap())
}
