use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CustomRetrieveValue {
    pub key: String,
}
