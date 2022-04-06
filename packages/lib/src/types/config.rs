#[derive(Debug)]
pub struct Config {
    pub table_name: String,
    pub aws_sdk_region: String,
    pub aws_sdk_endpoint: Option<String>,
}

impl Config {
    pub async fn aws_config(&self) -> aws_types::SdkConfig {
        aws_config::load_from_env().await
    }
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

    pub fn build(self) -> Config {
        Config {
            table_name: self.table_name,
            aws_sdk_endpoint: self.aws_sdk_endpoint,
            aws_sdk_region: self.aws_sdk_region,
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        ConfigBuilder::new()
    }
}
