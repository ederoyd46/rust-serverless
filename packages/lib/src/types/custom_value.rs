use rusoto_dynamodb::AttributeValue;
use serde_derive::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

use super::Storable;

#[derive(Clone, Eq, PartialEq, Deserialize, Debug)]
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
        if self.key().is_empty() {
            false
        } else {
            true
        }
    }

    fn get_pk(&self) -> String {
        self.key().to_string()
    }

    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();

        item.insert(
            "value".to_string(),
            AttributeValue {
                s: Some(self.value().to_string()),
                ..Default::default()
            },
        );

        item
    }
}
