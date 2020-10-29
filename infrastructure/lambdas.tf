# Store Lambda
resource aws_lambda_function store {
    function_name = "store"
    handler = "does.not.matter"
    runtime = "provided"
    filename = "deploy/store.zip"
    source_code_hash = filebase64sha256("deploy/store.zip")
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

resource aws_cloudwatch_log_group store {
  name              = "/aws/lambda/${aws_lambda_function.store.function_name}"
  retention_in_days = 3
}

output store_lambda {
  value = aws_lambda_function.store.arn
}

output store_lambda_log_group {
  value = aws_cloudwatch_log_group.store.arn
}
