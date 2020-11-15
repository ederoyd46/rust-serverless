mod custom_event;
mod custom_output;
mod custom_value;

pub use custom_event::CustomEvent;
pub use custom_value::CustomValue;
use serde_derive::Deserialize;

pub use custom_output::CustomOutput;

use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait Storable: Send + Sync {
    fn is_valid(&self) -> bool;
    fn get_pk(&self) -> String;
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue>;
}

#[derive(Clone, Eq, PartialEq, Deserialize)]
pub enum CustomType {
    CustomEvent(CustomEvent),
    CustomValue(CustomValue),
}
