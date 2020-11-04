use rusoto_dynamodb::AttributeValue;
use std::collections::HashMap;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;