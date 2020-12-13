provider "aws" {
  region = var.region
}

# terraform {
#   backend "s3" {
#     key = "46/terraform"
#     encrypt = true
#     bucket = "ederoyd"
#     region = "eu-central-1"
#   }
# }
