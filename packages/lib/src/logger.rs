use log::{debug, LevelFilter};
use simple_logger::SimpleLogger;

use crate::types::Error;

use once_cell::sync::OnceCell;

static INITIALISED_LOGGER: OnceCell<bool> = OnceCell::new();

pub fn initialise_logger() -> Result<(), Error> {
    if INITIALISED_LOGGER.get().is_none() {
        SimpleLogger::new()
            .with_module_level("store", LevelFilter::Debug)
            .with_module_level("lib::database", LevelFilter::Debug)
            .with_level(LevelFilter::Info)
            .init()?;
        INITIALISED_LOGGER.set(true).unwrap();
        debug!("Initialised Logger");
    }
    Ok(())
}
