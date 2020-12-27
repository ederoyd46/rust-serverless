# Root configurations - see https://learn.hashicorp.com/tutorials/terraform/lambda-api-gateway

resource aws_api_gateway_rest_api api {
  name        = "RustServerlessAPI"
  description = "RustServerless Rest API Gateway"
}

resource aws_api_gateway_deployment value {
   depends_on = [
     aws_api_gateway_integration.store_value,
     aws_api_gateway_integration.retrieve_value,
   ]

   rest_api_id = aws_api_gateway_rest_api.api.id
   stage_name  = "dev"
}

resource aws_api_gateway_resource db {
   rest_api_id = aws_api_gateway_rest_api.api.id
   parent_id   = aws_api_gateway_rest_api.api.root_resource_id
   path_part   = "db"
}

resource aws_api_gateway_resource db_key {
   rest_api_id = aws_api_gateway_rest_api.api.id
   parent_id   = aws_api_gateway_resource.db.id
   path_part   = "{key}"
}

output "base_url" {
  value = aws_api_gateway_deployment.value.invoke_url
}