mod config;
mod custom_retrieve_value;
mod custom_value;
mod error;

pub use config::*;
pub use custom_retrieve_value::CustomRetrieveValue;
pub use custom_value::CustomValue;
pub use error::AppError;

use aws_sdk_dynamodb::model::AttributeValue;
use std::collections::HashMap;

pub trait Storable: Send + Sync {
    fn is_valid(&self) -> bool;
    fn get_pk(&self) -> AttributeValue;
    fn to_dynamo_db(&self) -> AttributeValue;
}

pub trait Retrievable<T>: Send + Sync {
    fn from_dynamo_db(data: HashMap<String, AttributeValue>) -> Option<T>;
}
