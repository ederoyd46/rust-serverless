use rusoto_dynamodb::AttributeValue;
use serde_derive::Deserialize;
use std::collections::HashMap;
use serde_json::Value;

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
    fn get_pk(&self) -> String {
        self.key().to_string()
    }

    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert(
            "PK".to_string(),
            AttributeValue {
                s: Some(self.key().to_string()),
                ..Default::default()
            },
        );
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