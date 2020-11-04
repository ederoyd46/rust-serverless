use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait Storable {
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue>;
}
