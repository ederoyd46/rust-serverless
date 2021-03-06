use rusoto_dynamodb::AttributeValue;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

use super::{Retrievable, Storable};

#[derive(Deserialize, Serialize, Debug)]
pub struct CustomValue {
    pub key: String,
    pub value: Value,
}

impl CustomValue {
    pub fn key(&self) -> &String {
        &self.key
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Retrievable<Self> for CustomValue {
    fn from_dynamo_db(data: HashMap<String, AttributeValue>) -> Option<Self> {
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
        vec![("value".to_string(), build_attribute_value(self.value()))]
            .into_iter()
            .collect()
    }
}

fn build_serde_value(attribute: &AttributeValue) -> Value {
    if attribute.s.is_some() {
        let val = attribute.s.as_ref().unwrap();
        Value::String(val.to_string())
    } else if attribute.n.is_some() {
        let val = attribute.n.as_ref().unwrap();
        // TODO Hate this..
        match val.parse::<i64>() {
            Ok(v) => Value::Number(Number::from(v)),
            Err(_) => Value::Number(Number::from_f64(val.parse::<f64>().unwrap()).unwrap()),
        }
    } else if attribute.bool.is_some() {
        let val = attribute.bool.unwrap();
        Value::Bool(val)
    } else if attribute.m.is_some() {
        let val = attribute.m.as_ref().unwrap();
        Value::Object(
            val.iter()
                .map(|(k, v)| (k.to_string(), build_serde_value(v)))
                .collect(),
        )
    } else if attribute.l.is_some() {
        let val = attribute.l.as_ref().unwrap();
        Value::Array(val.iter().map(|v| build_serde_value(v)).collect())
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
            m: Some(build_dynamodb_object(&val)),
            ..Default::default()
        },
        Value::Array(val) => AttributeValue {
            l: Some(build_dynamodb_array(&val)),
            ..Default::default()
        },
        Value::Null => AttributeValue {
            l: None,
            ..Default::default()
        },
    }
}

fn build_dynamodb_object(object: &Map<String, Value>) -> HashMap<String, AttributeValue> {
    object
        .iter()
        .map(|(k, v)| (k.to_string(), build_attribute_value(v)))
        .collect()
}

fn build_dynamodb_array(object: &[Value]) -> Vec<AttributeValue> {
    object.iter().map(|v| build_attribute_value(v)).collect()
}

#[cfg(test)]
mod tests {
    use super::build_attribute_value;
    use rusoto_dynamodb::AttributeValue;
    use serde_json::{Number, Value};

    #[test]
    fn test_build_string_attribute_value() {
        let value = Value::String("String Value".to_string());
        let expected = AttributeValue {
            s: Some("String Value".to_string()),
            ..Default::default()
        };
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_number_attribute_value() {
        let value = Value::Number(Number::from(100));
        let expected = AttributeValue {
            n: Some(100.to_string()),
            ..Default::default()
        };
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_boolean_attribute_value() {
        let value = Value::Bool(true);
        let expected = AttributeValue {
            bool: Some(true),
            ..Default::default()
        };
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_object_attribute_value() {
        let value = Value::Object(
            vec![("boolean".to_string(), Value::Bool(true))]
                .into_iter()
                .collect(),
        );

        let expected = AttributeValue {
            m: Some(
                vec![(
                    "boolean".to_string(),
                    AttributeValue {
                        bool: Some(true),
                        ..Default::default()
                    },
                )]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        };
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_list_attribute_value() {
        let list_value = vec![Value::Bool(true), Value::String("String Value".to_string())]
            .into_iter()
            .collect();

        let value = Value::Array(list_value);

        let expected = AttributeValue {
            l: Some(
                vec![
                    AttributeValue {
                        bool: Some(true),
                        ..Default::default()
                    },
                    AttributeValue {
                        s: Some("String Value".to_string()),
                        ..Default::default()
                    },
                ]
                .into_iter()
                .collect(),
            ),
            ..Default::default()
        };
        assert_eq!(build_attribute_value(&value), expected);
    }
}
