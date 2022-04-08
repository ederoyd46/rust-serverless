# Retrieve Lambda
# module "retrive_value" {
#   source  = "terraform-aws-modules/lambda/aws"
#   version = "3.1.0"

#   function_name                     = "retrieve_value-${var.stage}"
#   description                       = "Retrieve Value"
#   handler                           = "does.not.matter"
#   runtime                           = "provided"
#   publish                           = true
#   cloudwatch_logs_retention_in_days = 1
#   create_package                    = false
#   local_existing_package            = "../deploy/retrieve_value.zip"
#   memory_size                       = 128
#   timeout                           = 3


#   environment_variables = {
#     DATABASE = aws_dynamodb_table.data_store.name
#   }

#   attach_policy_statements = true
#   policy_statements = {
#     service_config = {
#       effect    = "Allow",
#       actions   = ["dynamodb:GetItem"]
#       resources = [aws_dynamodb_table.data_store.arn]
#     },
#   }

#   allowed_triggers = {
#     ApiGateway = {
#       service    = "apigateway"
#       source_arn = "${aws_api_gateway_rest_api.gateway.execution_arn}/*/*"
#     }
#   }

# }




resource "aws_lambda_function" "retrieve_value" {
  function_name    = "retrieve_value-${var.stage}"
  handler          = "does.not.matter"
  runtime          = "provided"
  filename         = "../deploy/retrieve_value.zip"
  source_code_hash = filebase64sha256("../deploy/retrieve_value.zip")
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

resource "aws_cloudwatch_log_group" "retrieve_value" {
  name              = "/aws/lambda/${aws_lambda_function.retrieve_value.function_name}"
  retention_in_days = 3
}

# API Gateway
# resource "aws_api_gateway_method" "retrieve_value" {
#   rest_api_id   = aws_api_gateway_rest_api.api.id
#   resource_id   = aws_api_gateway_resource.db_key.id
#   http_method   = "GET"
#   authorization = "NONE"

#   request_parameters = {
#     "method.request.path.key" = true
#   }
# }

# resource "aws_api_gateway_integration" "retrieve_value" {
#   rest_api_id = aws_api_gateway_rest_api.api.id
#   resource_id = aws_api_gateway_method.retrieve_value.resource_id
#   http_method = aws_api_gateway_method.retrieve_value.http_method

#   # TODO Does this always need to be a POST even though the API is GET
#   integration_http_method = "POST"
#   type                    = "AWS_PROXY"
#   uri                     = aws_lambda_function.retrieve_value.invoke_arn
# }

# resource "aws_lambda_permission" "retrieve_value" {
#   statement_id  = "AllowAPIGatewayInvoke"
#   action        = "lambda:InvokeFunction"
#   function_name = aws_lambda_function.retrieve_value.function_name
#   principal     = "apigateway.amazonaws.com"

#   # The "/*/*" portion grants access from any method on any resource
#   # within the API Gateway REST API.
#   source_arn = "${aws_api_gateway_rest_api.api.execution_arn}/*/*"
# }


resource "aws_lambda_function_url" "retrieve_value" {
  function_name      = aws_lambda_function.retrieve_value.function_name
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
output "retrieve_value_lambda" {
  value = aws_lambda_function.retrieve_value.arn
}

output "retrieve_value_lambda_log_group" {
  value = aws_cloudwatch_log_group.retrieve_value.arn
}

output "retrieve_value_url" {
  value = aws_lambda_function_url.retrieve_value.function_url
}
