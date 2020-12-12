use serde_derive::Serialize;

#[derive(Serialize, Debug)]
pub struct CustomOutput {
    pub status: i64,
    pub body: String,
}
