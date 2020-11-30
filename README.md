# rust-serverless
Learning the Rust language and how this would work in a serverless environment.

![Rust](https://github.com/ederoyd46/rust-serverless/workflows/Rust/badge.svg)

## Dependencies
- [Rustup](https://rustup.rs/)
- [Cross](https://crates.io/crates/cross)
- [Docker](https://www.docker.com/)
- [DynamoDB](https://aws.amazon.com/dynamodb/)
- [Terraform](https://www.terraform.io/)
- [AWS CLI](https://aws.amazon.com/cli/)

## Setup
_Most_ commands needed to use this project up have been added to the Makefile.

By default the project is compiled without Lambda support to run against a local DynamoDB instance running on `http://localhost:8000`. 

### Start Local DynamoDB
A docker-compose.yml has been provided. Run the command below to start a DynamoDB instance.

```sh
docker-compose up -d
```

### Create DynamoDB Table

Docker uses a volume for storage so this only needs to be run once.

```sh
make table.create
```

### Compile for Development

```sh
make build
```

### Compile with Lambda Support

```sh
make build.with.features
```
_NOTE - If you're working on a Mac this will not work if you deploy it to AWS._

### Release
This will compile with Lambda support for release to keep the binary size down. We create static binaries so we don't need to worry about glibc versions.

```sh
make release
```

###  Package
This will rename the binaries to bootstrap and package them into zip files ready to deploy to AWS. This target assumes you've used `release`.

```sh
make package
```

### Deploy
This target runs Terraform and deploys the Lambdas.

```sh
make deploy
```

### Build Package Deploy
A helper target which runs `build`, `package` and `deploy`

```sh
make build.package.deploy
```

### Other commands
Build the docker image to compile static binaries on (only needed if you're not using Linux)
```sh
make build.image
```

List DynamoDB tables.

```sh
make table.list
```

Scan DynamoDB table.
```sh
make table.scan
```

Remove DynamoDB table.
```sh
make table.remove
```

Terraform Plan
```sh
make plan
```

Remove Terraform resources.

```sh
make remove
```

Test Local Value Lambda.
```sh
make test.local.value
```

Test Local Event Lambda.
```sh
make test.local.event
```

Test AWS Value Lambda.
```sh
make test.lambda.value
```

Test AWS Event Lambda.
```sh
make test.lambda.event
```