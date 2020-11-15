# Store Lambda
resource aws_lambda_function store_value {
    function_name = "store_value-${var.stage}"
    handler = "does.not.matter"
    runtime = "provided"
    filename = "deploy/store_value.zip"
    source_code_hash = filebase64sha256("deploy/store_value.zip")
    role = aws_iam_role.base_lambda_role.arn

    environment {
      variables = {
        DATABASE = aws_dynamodb_table.data_store.name
      }
    }

    lifecycle {
      ignore_changes = [last_modified]
    }
}

resource aws_cloudwatch_log_group store_value {
  name              = "/aws/lambda/${aws_lambda_function.store_value.function_name}"
  retention_in_days = 3
}

output store_value_lambda {
  value = aws_lambda_function.store_value.arn
}

output store_value_lambda_log_group {
  value = aws_cloudwatch_log_group.store_value.arn
}
