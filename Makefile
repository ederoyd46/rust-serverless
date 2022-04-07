# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}
DATA_STORE_NAME=rust_serverless_store-$(STAGE)

AWS_CLI=aws
TERRAFORM=terraform -chdir=./infrastructure

CROSS_TARGET=x86_64-unknown-linux-musl
CROSS_COMPILE=x86_64-linux-musl-

# Tasks

.PHONY: deploy

# Build Locally
build: 
	@cargo build 

test:
	@cargo test

#  Terraform
plan:
	@$(TERRAFORM) plan -var stage=$(STAGE)

terraform.init:
	@$(TERRAFORM) init

deploy:
	@$(TERRAFORM) apply -var stage=$(STAGE) -auto-approve

remove:
	@$(TERRAFORM) destroy -var stage=$(STAGE) -auto-approve


release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --target=$(CROSS_TARGET) --release
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release
endif

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
		curl -X POST "$$API_URL/db/$$f" -d "@./etc/$$f"; \
	done;

test.lambda.retrieve.value:
	@API_URL=$(shell $(TERRAFORM) output base_url); \
	FILES="$(shell ls ./etc)"; \
	for i in $$FILES; \
	do \
		curl -X GET $$API_URL/db/$$i; \
	done;

# Table tasks
table.list:
	@$(AWS_CLI) dynamodb list-tables

table.scan:
	@$(AWS_CLI) dynamodb scan --table-name $(DATA_STORE_NAME) $(ENDPOINT)

# e.g make table.get KEY="bedford.json"
table.get:
	@$(AWS_CLI) dynamodb get-item --table-name $(DATA_STORE_NAME) --key '{"PK": {"S": "$(KEY)"}}'


tail.retrieve:
	@$(AWS_CLI) logs tail "/aws/lambda/retrieve_value-${USER}" --follow --format short

tail.store:
	@$(AWS_CLI) logs tail "/aws/lambda/store_value-${USER}" --follow --format short
