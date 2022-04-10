# rust-serverless
Learning the Rust language and how this would work in a serverless environment.

![Rust](https://github.com/ederoyd46/rust-serverless/workflows/Rust/badge.svg)

## Dependencies
- [Rustup](https://rustup.rs/)
- [DynamoDB](https://aws.amazon.com/dynamodb/)
- [Terraform](https://www.terraform.io/)
- [AWS CLI](https://aws.amazon.com/cli/)

## Setup
_Most_ commands needed to use this project up have been added to the Makefile.

### Set up Mac OS for Cross Compiling to Static Linux Binaries

It saves a considerable amount of time to cross compile on Mac OS instead of using Docker with Cross.

- Assuming you're using brew, run;
```sh
brew install musl-cross
```

- Update the config file in `$(HOME)/.cargo/config` to include a linker entry for the musl target.
```
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

- Make sure you've installed the correct toolchain with `rustup`
```sh
rustup component add rust-std-x86_64-unknown-linux-musl
```
### Set up Linux for Cross Compiling to Static Binaries

- If you're on Ubuntu, run; 
```sh
apt-get install -y musl musl-dev musl-tools
```

- Make sure you've installed the correct toolchain with `rustup`
```sh
rustup component add rust-std-x86_64-unknown-linux-musl
```

### Compile for Development

```sh
make build
```

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
List DynamoDB tables.

```sh
make table.list
```

Scan DynamoDB table.
```sh
make table.scan
```

Terraform Plan
```sh
make plan
```

Remove Terraform resources.

```sh
make remove
```

Test AWS Store Value Lambda.
```sh
make test.lambda.store.value
```

Test AWS Retrieve Value Lambda.
```sh
make test.lambda.retrieve.value
```

Tail the store value lambda logs
```sh
make tail.store_value
```

Tail the retrieve value lambda logs
```sh
make tail.retrieve_value
```
