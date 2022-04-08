provider "aws" {
  region = var.region
}

terraform {
  backend "s3" {
    key     = "46/terraform"
    encrypt = true
    bucket  = "ederoyd"
    region  = "eu-central-1"
  }

  required_version = "= 1.1.3"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "4.9.0"
    }
  }
}
