use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, info};
use serde_derive::{Deserialize, Serialize};
use simple_logger;
use rusoto_core::{Region};
use rusoto_s3::{S3, S3Client, PutObjectRequest};

#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, _c: Context) -> Result<CustomOutput, HandlerError> {
    let s3_client = S3Client::new(Region::EuCentral1);
    let result = match s3_client.put_object(PutObjectRequest {
        bucket: String::from("bucket"),
        key: e.first_name.clone(),
        body: Some(e.first_name.clone().into_bytes().into()),
        ..Default::default()
    }).sync() {
        Ok(item) => item,
        Err(e) => panic!("Error completing futures: {}", e),
    };

    info!("item: {:?}", result);

    Ok(CustomOutput {
        message: format!("Lambda completed for {}", e.first_name),
    })
}