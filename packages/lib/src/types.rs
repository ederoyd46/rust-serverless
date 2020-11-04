use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CustomEvent {
    #[serde(rename = "firstName")]
    pub first_name: String,

    #[serde(rename = "lastName")]
    pub last_name: String,
}

impl CustomEvent {
    pub fn get_name(self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[derive(Serialize, Debug)]
pub struct CustomOutput {
    pub message: String,
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
