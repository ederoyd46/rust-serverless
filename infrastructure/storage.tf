# Data Storage
resource aws_dynamodb_table data_store {
  name = "rust_serverless_store-${var.stage}"
  billing_mode = "PAY_PER_REQUEST"
  hash_key = "firstName"

  attribute {
    name = "firstName"
    type = "S"
  }
}

output data_store {
  value = aws_dynamodb_table.data_store.arn
}

output data_store_name {
  value = aws_dynamodb_table.data_store.name
}
