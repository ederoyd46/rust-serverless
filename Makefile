# Global
BASE_DIR=$(shell pwd)
UNAME_S=$(shell uname -s)
STAGE=${USER}

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
	@$(TERRAFORM) plan

terraform.init:
	@$(TERRAFORM) init

deploy:
	@$(TERRAFORM) apply -auto-approve

remove:
	@$(TERRAFORM) destroy -auto-approve


release:
ifeq ("$(UNAME_S)","Linux")
	@cargo build --target=$(CROSS_TARGET) --release
else
	@CROSS_COMPILE=$(CROSS_COMPILE) cargo build --target=$(CROSS_TARGET) --release
endif

package.store_value: 
	@mkdir -p deploy/store_value
	@cp target/$(CROSS_TARGET)/release/store_value deploy/store_value/bootstrap
	# @upx -9 deploy/store_value/bootstrap
	@zip -j -9 deploy/store_value.zip deploy/store_value/bootstrap

package.retrieve_value: 
	@mkdir -p deploy/retrieve_value
	@cp target/$(CROSS_TARGET)/release/retrieve_value deploy/retrieve_value/bootstrap
	# @upx -9 deploy/retrieve_value/bootstrap
	@zip -j -9 deploy/retrieve_value.zip deploy/retrieve_value/bootstrap

package: package.store_value package.retrieve_value

release.package.deploy: release package deploy


# TEST
test.lambda.store.value:
	@API_URL=$(shell $(TERRAFORM) output store_value_url); \
	FILES="$(shell ls ./etc)"; \
	echo $$FILES; \
	for f in $$FILES; \
	do \
		# curl -X POST "$$API_URL/db/$$f" -d "@./etc/$$f"; \
		curl -X POST "$$API_URL/$$f" -d "@./etc/$$f"; \
	done;

test.lambda.retrieve.value:
	@API_URL=$(shell $(TERRAFORM) output retrieve_value_url); \
	FILES="$(shell ls ./etc)"; \
	for i in $$FILES; \
	do \
		curl -X GET $$API_URL/$$i; \
	done;

# Table tasks
table.list:
	@$(AWS_CLI) dynamodb list-tables

table.scan:
	@DATA_STORE_NAME=$(shell $(TERRAFORM) output data_store_name); \
	$(AWS_CLI) dynamodb scan --table-name $$DATA_STORE_NAME

# e.g make table.get KEY="bedford.json"
table.get:
	@DATA_STORE_NAME=$(shell $(TERRAFORM) output data_store_name); \
	$(AWS_CLI) dynamodb get-item --table-name $$DATA_STORE_NAME --key '{"PK": {"S": "$(KEY)"}}'


tail.retrieve_value:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output retrieve_value_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short

tail.store_value:
	@LOG_GROUP_NAME=$(shell $(TERRAFORM) output store_value_lambda_log_group); \
	$(AWS_CLI) logs tail $$LOG_GROUP_NAME --follow --format short