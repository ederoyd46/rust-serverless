# Store Lambda
module "store_value" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "3.1.0"

  function_name                     = "store_value-${var.stage}"
  description                       = "Store Value"
  handler                           = "does.not.matter"
  runtime                           = "provided"
  publish                           = true
  cloudwatch_logs_retention_in_days = 1
  create_package                    = false
  local_existing_package            = "../deploy/store_value.zip"
  memory_size                       = 128
  timeout                           = 3


  environment_variables = {
    DATABASE = aws_dynamodb_table.data_store.name
  }

  attach_policy_statements = true
  policy_statements = {
    service_config = {
      effect    = "Allow",
      actions   = ["dynamodb:PutItem"]
      resources = [aws_dynamodb_table.data_store.arn]
    },
  }

  # allowed_triggers = {
  #   ApiGateway = {
  #     service    = "apigateway"
  #     source_arn = "${aws_api_gateway_rest_api.gateway.execution_arn}/*/*"
  #   }
  # }

}



# resource "aws_lambda_function" "store_value" {
#   function_name    = "store_value-${var.stage}"
#   handler          = "does.not.matter"
#   runtime          = "provided"
#   filename         = "../deploy/store_value.zip"
#   source_code_hash = filebase64sha256("../deploy/store_value.zip")
#   role             = aws_iam_role.base_lambda_role.arn

#   environment {
#     variables = {
#       DATABASE = aws_dynamodb_table.data_store.name
#     }
#   }
#   lifecycle {
#     ignore_changes = [last_modified]
#   }
# }

# resource "aws_cloudwatch_log_group" "store_value" {
#   name              = "/aws/lambda/${aws_lambda_function.store_value.function_name}"
#   retention_in_days = 3
# }

resource "aws_lambda_function_url" "store_value" {
  function_name = module.store_value.lambda_function_name
  # qualifier          = "db"
  authorization_type = "NONE"
  # authorization_type = "AWS_IAM"

  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["POST"]
    allow_headers     = ["date", "keep-alive"]
    expose_headers    = ["keep-alive", "date"]
    max_age           = 86400
  }
}

# Outputs
output "store_value_lambda" {
  value = module.store_value.lambda_function_arn
}

output "store_value_lambda_log_group" {
  value = module.store_value.lambda_cloudwatch_log_group_name
}

output "store_value_url" {
  value = aws_lambda_function_url.store_value.function_url
}
