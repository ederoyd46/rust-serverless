use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CustomRetrieveValue {
    #[serde(rename = "key")]
    pub key: String,
}
