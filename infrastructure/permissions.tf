# Generic Lambda Role to add policies to
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

# Cloudwatch Policy
data aws_iam_policy_document cloudwatch_policy_document {
  statement {
    effect = "Allow"
    actions = [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
    ]

    resources = ["*"]
  }
}

resource aws_iam_role_policy cloudwatch_policy {
  name = "cloudwatch_policy" 
  role = aws_iam_role.base_lambda_role.id
  policy = data.aws_iam_policy_document.cloudwatch_policy_document.json
}

# Database Policy
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

