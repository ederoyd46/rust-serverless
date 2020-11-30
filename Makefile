# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}
DATA_STORE_NAME=rust_serverless_store-$(STAGE)
HOSTNAME=$(shell hostname)
ENDPOINT=--endpoint-url http://$(HOSTNAME):8000

USE_LOCAL_AWS=false
AWS_CLI_VERSION=2.1.4

USE_LOCAL_TERRAFORM=false
TERRAFORM_VERSION=0.13.5

# Task conditionals
ifeq ($(USE_LOCAL_AWS), true)
	AWS_CLI=aws
else
	AWS_CLI=docker run --rm -it \
		--network=rust-serverless_dynamodb \
		--link rust-serverless_dynamodb_1:localhost \
		-v ~/.aws:/root/.aws amazon/aws-cli:$(AWS_CLI_VERSION)
endif

ifeq ($(USE_LOCAL_TERRAFORM), true)
	TERRAFORM=terraform
else
	TERRAFORM=docker run --rm -v ~/.aws:/root/.aws -v $(PWD):/workspace \
	-w /workspace hashicorp/terraform:$(TERRAFORM_VERSION)
endif

ifeq ("$(UNAME_S)","Linux")
	BASE64=base64 --wrap=0
else
	BASE64=base64
endif


# Tasks

.PHONY: deploy

# Build Locally
build: 
	@cargo build 

build_with_features: 
	@cargo build --all-features

release: 
	@cargo build --all-features --release

package: 
	@for i in store_event store_value retrieve_value; \
	do \
		mkdir -p deploy/$$i; \
		cp target/release/$$i deploy/$$i/bootstrap; \
		zip -j -9 deploy/$$i.zip deploy/$$i/bootstrap; \
	done;

test:
	@cargo test


#  Terraform
plan:
	@$(TERRAFORM) plan -var stage=$(STAGE) infrastructure

terraform.init:
	@$(TERRAFORM) init infrastructure

deploy:
	@$(TERRAFORM) apply -var stage=$(STAGE) -auto-approve infrastructure

remove:
	@$(TERRAFORM) destroy -var stage=$(STAGE) -auto-approve infrastructure


# Cross Compile for deployment to AWS
CROSS_TARGET=x86_64-unknown-linux-musl

cross.build.image:
	@docker build -t ederoyd46/rust:build - < Dockerfile

cross.build:
ifeq ("$(UNAME_S)","Linux")
	cargo build --all-features --target=$(CROSS_TARGET) --release
else
	cross build --all-features --jobs 2 --target=$(CROSS_TARGET) --release
endif

cross.package: 
	@for i in store_event store_value retrieve_value; \
	do \
		mkdir -p deploy/$$i; \
		cp target/$(CROSS_TARGET)/release/$$i deploy/$$i/bootstrap; \
		zip -j -9 deploy/$$i.zip deploy/$$i/bootstrap; \
	done;

cross.build.deploy: cross.build cross.package deploy


# TEST
test.lambda.event:
	@$(AWS_CLI) lambda invoke --function-name store_event-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test", "lastName": "User"}' | $(BASE64)) out.json | cat

test.lambda.value:
	@$(AWS_CLI) lambda invoke --function-name store_value-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{ "key": "Key Object", "value": { "valString": "Sub Value 1", "valNumber": 1, "valBool": true, "valObj": { "valString": "Sub Value 2" }, "valArray": [ { "valArray": ["Sub Array 1", "Sub Array 2"] }, "some array string", 1, true ] }}' | $(BASE64)) out.json | cat

test.lambda.retrieve.value:
	@$(AWS_CLI) lambda invoke --function-name retrieve_value-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo "Key Object" | $(BASE64)) out.json | cat

test.local.event:
	@for i in 1 2 3 4; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_event -- '{"firstName": "Test '$$i'", "lastName": "User"}'; \
	done;

test.local.value:
	@for i in 1; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Object '$$i'", "value": { "valString": "Sub Value 1", "valNumber": 1, "valBool": true, "valObj": { "valString": "Sub Value 2" }, "valArray": [ { "valArray": ["Sub Array 1", "Sub Array 2"] }, "some array string", 1, true ] }}'; \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Array '$$i'", "value": ["val 1","val 2"]}'; \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Bool '$$i'", "value": true}'; \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Number '$$i'", "value": 1}'; \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key String '$$i'", "value": "Value"}'; \
	done;

test.local.retrieve.value:
	@for i in 1; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin retrieve_value -- "Key Bool $$i"; \
	done;

# Table tasks (Local Only)
table.list:
	@$(AWS_CLI) dynamodb list-tables $(ENDPOINT)

table.scan:
	@$(AWS_CLI) dynamodb scan --table-name $(DATA_STORE_NAME) $(ENDPOINT)

table.create:
	@$(AWS_CLI) dynamodb create-table --table-name $(DATA_STORE_NAME) \
		--attribute-definitions \
			AttributeName=PK,AttributeType=S \
		--key-schema \
			AttributeName=PK,KeyType=HASH \
		--billing-mode PAY_PER_REQUEST \
		$(ENDPOINT)
			
table.remove: 
	@$(AWS_CLI) dynamodb delete-table --table-name $(DATA_STORE_NAME) $(ENDPOINT)
