# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}
DATA_STORE_NAME=rust_serverless_store-$(STAGE)
HOSTNAME=$(shell hostname)
ENDPOINT=--endpoint-url http://$(HOSTNAME):8000

USE_LOCAL_AWS=false
AWS_CLI_VERSION=2.1.14

USE_LOCAL_TERRAFORM=false
TERRAFORM_VERSION=0.14.3

# Use Docker to Cross compile Linux Binaries (this can be slow)
USE_DOCKER_CROSS_COMPILE=false

# Task conditionals
ifeq ($(USE_LOCAL_AWS), true)
	AWS_CLI=aws
else
	AWS_CLI=docker run --rm -it \
		--network=rust-serverless_dynamodb \
		--link rust-serverless_dynamodb_1:${HOSTNAME} \
		-v ~/.aws:/root/.aws \
		-v $(PWD):/workspace \
		-w /workspace \
		amazon/aws-cli:$(AWS_CLI_VERSION)
endif

ifeq ($(USE_LOCAL_TERRAFORM), true)
	TERRAFORM=terraform
else
	TERRAFORM=docker run --rm -it -v ~/.aws:/root/.aws -v $(PWD):/workspace \
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

build.with.features: 
	@cargo build --all-features

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
build.image:
	@docker build -t ederoyd46/rust:build - < Dockerfile

CROSS_TARGET=x86_64-unknown-linux-musl
CROSS_COMPILE=x86_64-linux-musl-
release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --all-features --target=$(CROSS_TARGET) --release
else
ifeq ("$(USE_DOCKER_CROSS_COMPILE)","true")
	@cross build --all-features --jobs 2 --target=$(CROSS_TARGET) --release
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --all-features --target=$(CROSS_TARGET) --release
endif
endif

# package: 
# 	@for i in store_value retrieve_value; \
# 	do \
# 		mkdir -p deploy/$$i; \
# 		cp target/$(CROSS_TARGET)/release/$$i deploy/$$i/bootstrap; \
# 		zip -j -9 deploy/$$i.zip deploy/$$i/bootstrap; \
# 	done;

package.store_value: 
	@mkdir -p deploy/store_value
	@cp target/$(CROSS_TARGET)/release/store_value deploy/store_value/bootstrap
	@zip -j -9 deploy/store_value.zip deploy/store_value/bootstrap

package.retrieve_value: 
	@mkdir -p deploy/retrieve_value
	@cp target/$(CROSS_TARGET)/release/retrieve_value deploy/retrieve_value/bootstrap
	@zip -j -9 deploy/retrieve_value.zip deploy/retrieve_value/bootstrap

package: package.store_value package.retrieve_value

build.package.deploy: release package deploy


# TEST
test.lambda.store.value:
	@API_URL=$(shell $(TERRAFORM) output base_url); \
	FILES="$(shell ls ./etc)"; \
	echo $$FILES; \
	for f in $$FILES; \
	do \
		curl -X POST "$$API_URL/store/$$f" -d "@./etc/$$f"; \
	done;

test.lambda.retrieve.value:
	@API_URL=$(shell $(TERRAFORM) output base_url); \
	FILES="$(shell ls ./etc)"; \
	for i in $$FILES; \
	do \
		curl -X POST $$API_URL/retrieve/$$i; \
	done;

test.local.store.value:
	FILES="$(shell ls ./etc)"; \
	for f in $$FILES; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- $$f ./etc/$$f; \
	done;

test.local.retrieve.value:
	FILES="$(shell ls ./etc)"; \
	for f in $$FILES; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin retrieve_value -- $$f; \
	done;

# Table tasks (Local Only)
table.list:
	@$(AWS_CLI) dynamodb list-tables $(ENDPOINT)

table.scan:
	@$(AWS_CLI) dynamodb scan --table-name $(DATA_STORE_NAME) $(ENDPOINT)

# e.g make table.get KEY="bedford.json"
table.get:
	@$(AWS_CLI) dynamodb get-item --table-name $(DATA_STORE_NAME) --key '{"PK": {"S": "$(KEY)"}}' $(ENDPOINT)

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
