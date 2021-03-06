mod custom_retrieve_value;
mod custom_value;
mod config;

pub use custom_retrieve_value::CustomRetrieveValue;
pub use custom_value::CustomValue;
pub use config::*;

use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub trait Storable: Send + Sync {
    fn is_valid(&self) -> bool;
    fn get_pk(&self) -> String;
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue>;
}

pub trait Retrievable<T>: Send + Sync {
    fn from_dynamo_db(data: HashMap<String, AttributeValue>) -> Option<T>;
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
