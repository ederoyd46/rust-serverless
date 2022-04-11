use aws_sdk_dynamodb::model::AttributeValue;
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
        let key = data.get("PK")?.as_s().unwrap();
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

    fn get_pk(&self) -> AttributeValue {
        AttributeValue::S(self.key().to_string())
    }

    fn to_dynamo_db(&self) -> AttributeValue {
        build_attribute_value(self.value())
    }
}

fn build_serde_value(attribute: &AttributeValue) -> Value {
    if attribute.is_s() {
        let val = attribute.as_s().unwrap();
        Value::String(val.to_string())
    } else if attribute.is_n() {
        let val = attribute.as_n().unwrap();
        // TODO Hate this..
        match val.parse::<i64>() {
            Ok(v) => Value::Number(Number::from(v)),
            Err(_) => Value::Number(Number::from_f64(val.parse::<f64>().unwrap()).unwrap()),
        }
    } else if attribute.is_bool() {
        let val = attribute.as_bool().unwrap().to_owned();
        Value::Bool(val)
    } else if attribute.is_m() {
        let val = attribute.as_m().unwrap();
        Value::Object(
            val.iter()
                .map(|(k, v)| (k.to_string(), build_serde_value(v)))
                .collect(),
        )
    } else if attribute.is_l() {
        let val = attribute.as_l().unwrap();
        Value::Array(val.iter().map(|v| build_serde_value(v)).collect())
    } else {
        Value::Null
    }
}

fn build_attribute_value(value: &Value) -> AttributeValue {
    match value {
        Value::String(val) => AttributeValue::S(val.to_string()),
        Value::Null => AttributeValue::Null(true),
        Value::Bool(val) => AttributeValue::Bool(val.clone()),
        Value::Number(val) => AttributeValue::N(val.to_string()),
        Value::Array(val) => build_dynamodb_array(val),
        Value::Object(val) => build_dynamodb_object(val),
    }
}

fn build_dynamodb_object(object: &Map<String, Value>) -> AttributeValue {
    AttributeValue::M(
        object
            .iter()
            .map(|(k, v)| (k.to_string(), build_attribute_value(v)))
            .collect(),
    )
}

fn build_dynamodb_array(object: &[Value]) -> AttributeValue {
    AttributeValue::L(object.iter().map(|v| build_attribute_value(v)).collect())
}

#[cfg(test)]
mod tests {
    use super::build_attribute_value;
    use aws_sdk_dynamodb::model::AttributeValue;
    use serde_json::{Number, Value};

    #[test]
    fn test_build_string_attribute_value() {
        let value = Value::String("String Value".to_string());
        let expected = AttributeValue::S("String Value".to_string());
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_number_attribute_value() {
        let value = Value::Number(Number::from(100));
        let expected = AttributeValue::N(100.to_string());
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_boolean_attribute_value() {
        let value = Value::Bool(true);
        let expected = AttributeValue::Bool(true);
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_object_attribute_value() {
        let value = Value::Object(
            vec![("boolean".to_string(), Value::Bool(true))]
                .into_iter()
                .collect(),
        );

        let expected = AttributeValue::M(
            vec![("boolean".to_string(), AttributeValue::Bool(true))]
                .into_iter()
                .collect(),
        );
        assert_eq!(build_attribute_value(&value), expected);
    }

    #[test]
    fn test_build_list_attribute_value() {
        let list_value = vec![Value::Bool(true), Value::String("String Value".to_string())]
            .into_iter()
            .collect();

        let value = Value::Array(list_value);

        let expected = AttributeValue::L(
            vec![
                AttributeValue::Bool(true),
                AttributeValue::S("String Value".to_string()),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(build_attribute_value(&value), expected);
    }
}
