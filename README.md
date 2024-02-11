# AWS_Lambda_Function
Creating a simple AWS Lambda function that processes data.

## Overview
This week's mini project was dedicated to understanding how to write an AWS lambda function in Rust. This function is a simple data processing one, which reads a CSV file locally, processes the data, and then writes the most common word in a specific column to the command line. 

## Prerequisites
- AWS CLI
- Rust
- AWS Lambda Rust Runtime

## Steps
1. Create a new Rust project using `cargo new <project_name>`. 
2. Add the following dependencies to the `Cargo.toml` file:
```toml
[dependencies]
lambda_http = "0.9.2"
lambda_runtime = "0.9.1"
tokio = { version = "1", features = ["macros"] }
tracing = { version = "0.1", features = ["log"] }
tracing-subscriber = { version = "0.3", default-features = false, features = ["env-filter", "fmt"] }
csv = "1.1.6"
```
3. Create a new file called [`main.rs`]() which has the data processing function wrapped in a lambda handler. 
4. Test the function locally using `cargo run`. Personally, I first created the data processing function and tested it locally before wrapping it in a lambda handler. This helped me understand the logic and the flow of the function, and ensured that the function worked as expected.
5. Thenafter creating the lambda handler, I tested the function locally using `cargo lambda watch` to ensure that the lambda handler worked as expected. This command watches for changes in the code and automatically updates the lambda function, and you can see the output you would usually see in the terminal in a local environment on your browser. 
<img width="576" alt="Screenshot 2024-02-09 at 11 25 37 AM" src="https://github.com/nogibjj/aad64_Pandas-Script/assets/143753050/3202d552-c588-4dee-b5dc-a305ce883d70">

![WhatsApp Image 2024-02-09 at 11 25 24 AM](https://github.com/nogibjj/aad64_Pandas-Script/assets/143753050/c890fad5-38fa-4084-861b-e2dbd7fdf187)

6. After making sure that this was working, you set up your lambda function on AWS.

## Setting up the Lambda function
1. First, you need to create a new IAM role with the following permissions:
- AWSLambdaBasicExecutionRole
- AmazonAPIGatewayAdministrator	
- AmazonAPIGatewayInvokeFullAccess	
- AmazonAPIGatewayPushToCloudWatchLogs	
- AmazonS3ObjectLambdaExecutionRolePolicy	
- AWSCodeDeployRoleForLambda	
- AWSLambda_FullAccess	
- AWSLambdaDynamoDBExecutionRole	
- AWSLambdaInvocation-DynamoDB	
- CloudWatchLambdaInsightsExecutionRolePolicy	
- CloudWatchLogsFullAccess
2. Then you want to add some inline policies to the role:
- iam:AttachRolePolicy
- iam:CreateRole
- iam:PassRole
- iam:UpdateAssumeRolePolicy
- lambda:CreateFunction
- lambda:GetFunction
3. Then you want to configure the AWS CLI with your credentials using `aws configure` in your terminal. Make sure to have you AWS Access Key ID and Secret Access Key ready. Don't forget to set the default region to the one you want to deploy your lambda function to.

## Deploying the Lambda function
1. First, you need to create a zip file of your lambda function using `cargo lambda build`. This command creates a zip file of your lambda function and its dependencies.
2. Then you need to create a new lambda function using the AWS CLI. You can do this by running the following command:
```bash
aws lambda update-function-code --function-name your_lambda_function_name --zip-file fileb://target/lambda/<your-lambda-function-name>/bootstrap.zip
```
3. After this, you can test your lambda function using the AWS console or the AWS CLI. You can check the CloudWatch logs to see the output of your lambda function. 
<img width="1619" alt="Screenshot 2024-02-10 at 12 56 33 PM" src="https://github.com/nogibjj/aad64_Pandas-Script/assets/143753050/92490ae5-4fc0-4e60-a984-6c4d7b4a8c9e">
<img width="1415" alt="Screenshot 2024-02-10 at 1 55 35 PM" src="https://github.com/nogibjj/aad64_Pandas-Script/assets/143753050/a499d2a5-7041-4568-932d-6dad7920df95">

4. If all is running well, you'll be able to trigger your lambda function using the url provided in the AWS console and the output will be displayed in your browser. 

<img width="1711" alt="Screenshot 2024-02-10 at 1 54 19 PM" src="https://github.com/nogibjj/aad64_Pandas-Script/assets/143753050/d1a9bd17-a2f1-4798-9145-91eb46b096b4">