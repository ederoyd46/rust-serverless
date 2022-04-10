# Data Storage
resource aws_dynamodb_table data_store {
  name = "rust_serverless_store-${terraform.workspace}"
  billing_mode = "PAY_PER_REQUEST"
  hash_key = "PK"

  attribute {
    name = "PK"
    type = "S"
  }
}

output data_store {
  value = aws_dynamodb_table.data_store.arn
}

output data_store_name {
  value = aws_dynamodb_table.data_store.name
}
