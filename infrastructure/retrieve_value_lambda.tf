# Retrieve Lambda
module "retrieve_value" {
  source  = "terraform-aws-modules/lambda/aws"
  version = "3.1.0"

  function_name                     = "retrieve_value-${terraform.workspace}"
  description                       = "Retrieve Value"
  handler                           = "does.not.matter"
  runtime                           = "provided"
  publish                           = true
  cloudwatch_logs_retention_in_days = 1
  create_package                    = false
  local_existing_package            = "../deploy/retrieve_value.zip"
  memory_size                       = 128
  timeout                           = 3


  environment_variables = {
    DATABASE = aws_dynamodb_table.data_store.name
  }

  attach_policy_statements = true
  policy_statements = {
    service_config = {
      effect    = "Allow",
      actions   = ["dynamodb:GetItem"]
      resources = [aws_dynamodb_table.data_store.arn]
    },
  }

  #   allowed_triggers = {
  #     ApiGateway = {
  #       service    = "apigateway"
  #       source_arn = "${aws_api_gateway_rest_api.gateway.execution_arn}/*/*"
  #     }
  #   }

}

resource "aws_lambda_function_url" "retrieve_value" {
  function_name = module.retrieve_value.lambda_function_name
  # qualifier          = "db"
  authorization_type = "NONE"
  # authorization_type = "AWS_IAM"
  cors {
    allow_credentials = true
    allow_origins     = ["*"]
    allow_methods     = ["GET"]
    allow_headers     = ["date", "keep-alive"]
    expose_headers    = ["keep-alive", "date"]
    max_age           = 86400
  }
}

# Outputs
output "retrieve_value_lambda" {
  value = module.retrieve_value.lambda_function_arn
}

output "retrieve_value_lambda_log_group" {
  value = module.retrieve_value.lambda_cloudwatch_log_group_name
}

output "retrieve_value_url" {
  value = aws_lambda_function_url.retrieve_value.function_url
}
