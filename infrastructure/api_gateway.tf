# Root configurations - see https://learn.hashicorp.com/tutorials/terraform/lambda-api-gateway

resource aws_api_gateway_rest_api api {
  name        = "RustServerlessAPI"
  description = "RustServerless Rest API Gateway"
}

resource aws_api_gateway_deployment store_value {
   depends_on = [
     aws_api_gateway_integration.store_value,
     aws_api_gateway_integration.retrieve_value,
   ]

   rest_api_id = aws_api_gateway_rest_api.api.id
   stage_name  = "dev"
}

output "base_url" {
  value = aws_api_gateway_deployment.store_value.invoke_url
}





# resource aws_api_gateway_resource store_value {
#    rest_api_id = aws_api_gateway_rest_api.api.id
#    parent_id   = aws_api_gateway_rest_api.api.root_resource_id
#    path_part   = "{proxy+}"
# }

# resource aws_api_gateway_method store_value {
#    rest_api_id   = aws_api_gateway_rest_api.api.id
#    resource_id   = aws_api_gateway_resource.store_value.id
#    http_method   = "POST"
#    authorization = "NONE"
# }

# resource aws_api_gateway_integration store_value {
#    rest_api_id = aws_api_gateway_rest_api.api.id
#    resource_id = aws_api_gateway_method.store_value.resource_id
#    http_method = aws_api_gateway_method.store_value.http_method

#    integration_http_method = "POST"
#    type                    = "AWS_PROXY"
#    uri                     = aws_lambda_function.store_value.invoke_arn
# }
