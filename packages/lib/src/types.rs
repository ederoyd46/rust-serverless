mod custom_event;
mod custom_output;

pub use custom_event::CustomEvent;
pub use custom_output::CustomOutput;

use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait Storable: Send + Sync {
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue>;
}
