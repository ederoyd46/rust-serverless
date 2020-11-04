
use rusoto_dynamodb::AttributeValue;
use serde_derive::{Deserialize};
use std::collections::HashMap;

pub trait Storable {
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue>;
}

#[derive(Deserialize, Debug)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
}

impl CustomEvent {
    pub fn get_name(&self) -> String {
        if self.last_name.is_some() {
            format!("{} {}", self.first_name, self.last_name.as_ref().unwrap())
        } else {
            format!("{}", self.first_name)
        }
    }
}

impl Storable for CustomEvent {
    fn to_dynamo_db(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
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