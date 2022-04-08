# Store Lambda
resource "aws_lambda_function" "store_value" {
  function_name    = "store_value-${var.stage}"
  handler          = "does.not.matter"
  runtime          = "provided"
  filename         = "../deploy/store_value.zip"
  source_code_hash = filebase64sha256("../deploy/store_value.zip")
  role             = aws_iam_role.base_lambda_role.arn

  environment {
    variables = {
      DATABASE = aws_dynamodb_table.data_store.name
    }
  }
  lifecycle {
    ignore_changes = [last_modified]
  }
}

resource "aws_cloudwatch_log_group" "store_value" {
  name              = "/aws/lambda/${aws_lambda_function.store_value.function_name}"
  retention_in_days = 3
}

# API Gateway
# resource "aws_api_gateway_method" "store_value" {
#   rest_api_id   = aws_api_gateway_rest_api.api.id
#   resource_id   = aws_api_gateway_resource.db_key.id
#   http_method   = "POST"
#   authorization = "NONE"

#   request_parameters = {
#     "method.request.path.key" = true
#   }
# }

# resource "aws_api_gateway_integration" "store_value" {
#   rest_api_id = aws_api_gateway_rest_api.api.id
#   resource_id = aws_api_gateway_method.store_value.resource_id
#   http_method = aws_api_gateway_method.store_value.http_method

#   integration_http_method = "POST"
#   type                    = "AWS_PROXY"
#   uri                     = aws_lambda_function.store_value.invoke_arn
# }

# resource "aws_lambda_permission" "store_value" {
#   statement_id  = "AllowAPIGatewayInvoke"
#   action        = "lambda:InvokeFunction"
#   function_name = aws_lambda_function.store_value.function_name
#   principal     = "apigateway.amazonaws.com"

#   # The "/*/*" portion grants access from any method on any resource
#   # within the API Gateway REST API.
#   source_arn = "${aws_api_gateway_rest_api.api.execution_arn}/*/*"
# }




resource "aws_lambda_function_url" "store_value" {
  function_name      = aws_lambda_function.store_value.function_name
  authorization_type = "NONE"
  # cors {
  #   allow_credentials = true
  #   allow_origins     = ["*"]
  #   allow_methods     = ["POST"]
  #   allow_headers     = ["date", "keep-alive"]
  #   expose_headers    = ["keep-alive", "date"]
  #   max_age           = 86400
  # }
  #   qualifier          = "my_alias"
  #   authorization_type = "AWS_IAM"
}

# Outputs
output "store_value_lambda" {
  value = aws_lambda_function.store_value.arn
}

output "store_value_lambda_log_group" {
  value = aws_cloudwatch_log_group.store_value.arn
}

output "store_value_url" {
  value = aws_lambda_function_url.store_value.function_url

}
