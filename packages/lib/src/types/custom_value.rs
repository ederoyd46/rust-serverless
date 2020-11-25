use rusoto_dynamodb::AttributeValue;
use serde_derive::Deserialize;
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::Storable;

#[derive(Deserialize, Debug)]
pub struct CustomValue {
    #[serde(rename = "key")]
    key: String,

    #[serde(rename = "value")]
    value: Value,
}

impl CustomValue {
    pub fn key(&self) -> &String {
        &self.key
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Storable for CustomValue {
    fn is_valid(&self) -> bool {
        !self.key().is_empty()
    }

    fn get_pk(&self) -> String {
        self.key().to_string()
    }

    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert("value".to_string(), build_attribute_value(self.value()));
        item
    }
}

fn build_attribute_value(value: &Value) -> AttributeValue {
    match value {
        Value::String(val) => AttributeValue {
            s: Some(val.to_string()),
            ..Default::default()
        },
        Value::Number(val) => AttributeValue {
            n: Some(val.to_string()),
            ..Default::default()
        },
        Value::Bool(val) => AttributeValue {
            bool: Some(*val),
            ..Default::default()
        },
        Value::Object(val) => AttributeValue {
            m: Some(build_dynamodb_object(val.clone())),
            ..Default::default()
        },
        Value::Array(val) => AttributeValue {
            l: Some(build_dynamodb_array(val.clone())),
            ..Default::default()
        },
        Value::Null => AttributeValue {
            l: None,
            ..Default::default()
        },
    }
}

fn build_dynamodb_object(object: Map<String, Value>) -> HashMap<String, AttributeValue> {
    let mut items = HashMap::new();

    for (k, v) in object.iter() {
        items.insert(k.to_string(), build_attribute_value(v));
    }

    items
}

fn build_dynamodb_array(object: Vec<Value>) -> Vec<AttributeValue> {
    let mut items = vec!();

    for v in object.iter() {
        items.push(build_attribute_value(v))
    }

    items
}
