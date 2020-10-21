provider "aws" {
  region = "eu-central-1"
  version = "~> 3.9.0"
}

terraform {
  backend "s3" {
    key = "46/terraform"
    encrypt = true
    bucket = "ederoyd"
    region = "eu-central-1"
  }
}

# Data Storage
resource "aws_dynamodb_table" data_store {
  name = "rust_serverless_store"
  billing_mode = "PAY_PER_REQUEST"
  hash_key = "firstName"

  attribute {
    name = "firstName"
    type = "S"
  }
}

data aws_iam_policy_document data_store_policy_document {
  statement {
    effect = "Allow"
    actions = [
      "dynamodb:UpdateItem",
    ]

    resources = [aws_dynamodb_table.data_store.arn]
  }
}

resource aws_iam_role_policy data_store_role_policy {
  name   = "data_store_role_policy"
  role = aws_iam_role.base_lambda_role.id
  policy = data.aws_iam_policy_document.data_store_policy_document.json
}

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

# Generic Lambda Role
resource aws_iam_role base_lambda_role {
  name = "base_lambda_role" 
  assume_role_policy = <<-EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource aws_iam_role_policy cloudwatch_policy {
  name = "cloudwatch_policy" 
  role = aws_iam_role.base_lambda_role.id
  policy = <<-EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Effect": "Allow",
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "*"
    }
  ]
}
EOF
}



# resource aws_iam_policy "data_store_policy" {
#   name   = "data_store_policy"
#   role = 
#   policy = data.aws_iam_policy_document.data_store_policy_document.json
#   # path   = "/"
# }




# resource aws_iam_role_policy dynamodb_policy {
#   name = "dynamodb_policy" 
#   role = aws_iam_role.base_lambda_role.id
#   policy = <<-EOF
# {
#   "Version": "2012-10-17",
#   "Statement": [
#     {
#       "Effect": "Allow",
#       "Action": [
#         "dynamodb:PutItem",
#         "dynamodb:UpdateItem",
#         "dynamodb:GetItem"
#       ],
#       "Resource": aws_dynamodb_table.data_store
#     }
#   ]
# }
# EOF
# }

