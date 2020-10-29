provider "aws" {
  region = var.region
  version = "~> 3.9.0"
}

terraform {
  backend "s3" {
    key = "46/terraform"
    encrypt = true
    bucket = "ederoyd"
    region = "eu-central-1"
  }
}
