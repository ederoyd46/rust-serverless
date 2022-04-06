use log::{debug, LevelFilter};

use crate::types::Error;

use once_cell::sync::OnceCell;

static INITIALISED_LOGGER: OnceCell<bool> = OnceCell::new();

pub fn initialise_logger() -> Result<(), Error> {
    if INITIALISED_LOGGER.get().is_none() {
        env_logger::builder()
            .filter(Some("store_value"), LevelFilter::Debug)
            .filter(Some("retrieve_value"), LevelFilter::Debug)
            .filter(Some("lib::database"), LevelFilter::Debug)
            .filter_level(LevelFilter::Info)
            .init();
        INITIALISED_LOGGER.set(true).unwrap();
        debug!("Initialised Logger");
    }
    Ok(())
}
