use rusoto_dynamodb::AttributeValue;
use serde_derive::Deserialize;
use std::collections::HashMap;

use super::Storable;

#[derive(Deserialize, Debug)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,

    #[serde(rename = "lastName")]
    last_name: Option<String>,
}

impl CustomEvent {
    pub fn name(&self) -> String {
        if self.last_name.is_some() {
            format!("{} {}", self.first_name, self.last_name.as_ref().unwrap())
        } else {
            format!("{}", self.first_name)
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.first_name.is_empty() {
            false
        } else {
            true
        }
    }
}

impl Storable for CustomEvent {
    fn get_pk(&self) -> String {
        self.name()
    }

    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert(
            "PK".to_string(),
            AttributeValue {
                s: Some(self.get_pk().to_string()),
                ..Default::default()
            },
        );

        item.insert(
            "firstName".to_string(),
            AttributeValue {
                s: Some(self.first_name.to_string()),
                ..Default::default()
            },
        );

        if self.last_name.is_some() {
            item.insert(
                "lastName".to_string(),
                AttributeValue {
                    s: Some(self.last_name.as_ref().unwrap().to_string()),
                    ..Default::default()
                },
            );
        }
        item
    }
}
