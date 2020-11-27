# Retrieve Lambda
resource aws_lambda_function retrieve_value {
    function_name = "retrieve_value-${var.stage}"
    handler = "does.not.matter"
    runtime = "provided"
    filename = "deploy/retrieve_value.zip"
    source_code_hash = filebase64sha256("deploy/retrieve_value.zip")
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

resource aws_cloudwatch_log_group retrieve_value {
  name              = "/aws/lambda/${aws_lambda_function.retrieve_value.function_name}"
  retention_in_days = 3
}

output retrieve_value_lambda {
  value = aws_lambda_function.retrieve_value.arn
}

output retrieve_value_lambda_log_group {
  value = aws_cloudwatch_log_group.retrieve_value.arn
}
