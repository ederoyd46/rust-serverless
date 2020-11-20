mod custom_event;
mod custom_output;
mod custom_value;

pub use custom_event::CustomEvent;
pub use custom_value::CustomValue;

pub use custom_output::CustomOutput;

use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait Storable: Send + Sync {
    fn is_valid(&self) -> bool;
    fn get_pk(&self) -> String;
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue>;
}
