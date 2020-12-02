use rusoto_dynamodb::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::Storable;

#[derive(Deserialize, Serialize, Debug)]
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

    // TODO Move to Storable
    pub fn from_dynamo_db(data: HashMap<String, AttributeValue>) -> Option<Self> {
        let key = data.get("PK")?.s.as_ref()?;
        let value = build_serde_value(data.get("value")?);
        Some(Self {
            key: key.to_string(),
            value,
        })
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

// TODO Handle objects
fn build_serde_value(attribute: &AttributeValue) -> Value {
    if attribute.s.is_some() {
        let val = attribute.s.as_ref().unwrap();
        Value::String(val.to_string())
    } else if attribute.n.is_some() {
        let val = attribute.n.as_ref().unwrap();
        Value::Number(serde_json::Number::from(val.parse::<i64>().unwrap()))
    } else if attribute.bool.is_some() {
        let val = attribute.bool.unwrap();
        Value::Bool(val)
    } else if attribute.m.is_some() {
        let val = attribute.m.as_ref().unwrap();
        let mut object = serde_json::Map::new();
        for (k, v) in val.iter() {
            object.insert(k.to_string(), build_serde_value(v));
        }
        Value::Object(object)
    } else if attribute.l.is_some() {
        let val = attribute.l.as_ref().unwrap();

        let mut items = vec![];
        for v in val.iter() {
            items.push(build_serde_value(v))
        }

        Value::Array(items)
    } else {
        Value::Null
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
    let mut items = vec![];

    for v in object.iter() {
        items.push(build_attribute_value(v))
    }

    items
}
