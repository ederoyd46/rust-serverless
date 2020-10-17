use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    pub first_name: String,
}

#[derive(Serialize, Debug)]
pub struct CustomOutput {
    pub message: String,
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
