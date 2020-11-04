use serde_derive::{Serialize};

#[derive(Serialize, Debug)]
pub struct CustomOutput {
    pub message: String,
}
