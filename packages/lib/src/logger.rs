use log::{LevelFilter};

use crate::types::Error;

pub fn initialise_logger() -> Result<(), Error> {
    env_logger::builder()
        .filter(Some("store_value"), LevelFilter::Debug)
        .filter(Some("retrieve_value"), LevelFilter::Debug)
        .filter(Some("lib::database"), LevelFilter::Debug)
        .filter_level(LevelFilter::Info)
        .init();
    Ok(())
}
