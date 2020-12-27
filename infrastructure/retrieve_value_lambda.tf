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

# API Gateway

resource aws_api_gateway_resource retrieve_value {
   rest_api_id = aws_api_gateway_rest_api.api.id
   parent_id   = aws_api_gateway_rest_api.api.root_resource_id
   path_part   = "retrieve"
}
resource aws_api_gateway_resource retrieve_value_key {
   rest_api_id = aws_api_gateway_rest_api.api.id
   parent_id   = aws_api_gateway_resource.retrieve_value.id
   path_part   = "{key}"
}

resource aws_api_gateway_method retrieve_value {
   rest_api_id   = aws_api_gateway_rest_api.api.id
   resource_id   = aws_api_gateway_resource.retrieve_value_key.id
   http_method   = "POST"
   authorization = "NONE"

  request_parameters = {
    "method.request.path.key" = true
  }   
}

resource aws_api_gateway_integration retrieve_value {
   rest_api_id = aws_api_gateway_rest_api.api.id
   resource_id = aws_api_gateway_method.retrieve_value.resource_id
   http_method = aws_api_gateway_method.retrieve_value.http_method

   integration_http_method = "POST"
   type                    = "AWS_PROXY"
   uri                     = aws_lambda_function.retrieve_value.invoke_arn
}

resource aws_lambda_permission retrieve_value {
   statement_id  = "AllowAPIGatewayInvoke"
   action        = "lambda:InvokeFunction"
   function_name = aws_lambda_function.retrieve_value.function_name
   principal     = "apigateway.amazonaws.com"

   # The "/*/*" portion grants access from any method on any resource
   # within the API Gateway REST API.
   source_arn = "${aws_api_gateway_rest_api.api.execution_arn}/*/*"
}

# Outputs
output retrieve_value_lambda {
  value = aws_lambda_function.retrieve_value.arn
}

output retrieve_value_lambda_log_group {
  value = aws_cloudwatch_log_group.retrieve_value.arn
}
