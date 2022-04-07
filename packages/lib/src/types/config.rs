use aws_sdk_dynamodb::Client;
use aws_types::SdkConfig;
#[derive(Debug)]
pub struct Config {
    pub table_name: String,
    pub aws_sdk_config: SdkConfig,
    pub dynamodb: Client,
}

pub struct ConfigBuilder {
    table_name: String,
    aws_sdk_region: String,
    aws_sdk_endpoint: Option<String>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            table_name: "default_table".to_string(),
            aws_sdk_region: "eu-central-1".to_string(),
            aws_sdk_endpoint: None,
        }
    }

    pub fn table_name(mut self, name: String) -> Self {
        self.table_name = name;
        self
    }

    pub fn aws_sdk_endpoint(mut self, endpoint: Option<String>) -> Self {
        self.aws_sdk_endpoint = endpoint;
        self
    }

    pub fn aws_sdk_region(mut self, region: String) -> Self {
        self.aws_sdk_region = region;
        self
    }
    pub async fn build(self) -> Config {
        let aws_sdk_config = aws_config::load_from_env().await; //TODO take the region and endpoint above into consideration
        Config {
            table_name: self.table_name,
            dynamodb: aws_sdk_dynamodb::Client::new(&aws_sdk_config),
            aws_sdk_config,
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}
