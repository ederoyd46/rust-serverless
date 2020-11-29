BASE_DIR=$(shell pwd)
STAGE=${USER}
DATA_STORE_NAME=rust_serverless_store-$(STAGE)
ENDPOINT=--endpoint-url http://local.data:8000

AWS_CLI_VERSION=2.1.4
TERRAFORM_VERSION=0.13.5

# Not convinced this is a good idea...
AWS_CLI=docker run --rm -it \
	--network=rust-serverless_dynamodb \
	--link rust-serverless_dynamodb_1:local.data \
	-v ~/.aws:/root/.aws amazon/aws-cli:$(AWS_CLI_VERSION)

TERRAFORM=docker run --rm -v ~/.aws:/root/.aws -v $(PWD):/workspace -w /workspace hashicorp/terraform:$(TERRAFORM_VERSION)

.PHONY: deploy

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

plan:
	@$(TERRAFORM) plan -var stage=$(STAGE) infrastructure

terraform.init:
	@$(TERRAFORM) init infrastructure

deploy:
	@$(TERRAFORM) apply -var stage=$(STAGE) -auto-approve infrastructure

remove:
	@$(TERRAFORM) destroy -var stage=$(STAGE) -auto-approve infrastructure

test:
	@cargo test

cross.build: 
	cross build --all-features --jobs 1 --target=x86_64-unknown-linux-gnu --release

cross.package: 
	@for i in store_event store_value retrieve_value; \
	do \
		mkdir -p deploy/$$i; \
		cp target/x86_64-unknown-linux-gnu/release/$$i deploy/$$i/bootstrap; \
		zip -j -9 deploy/$$i.zip deploy/$$i/bootstrap; \
	done;

cross.build.deploy: cross.build cross.package deploy

test.lambda.event:
	@aws lambda invoke --function-name store_event-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test", "lastName": "User"}' | base64) out.json | cat

test.lambda.value:
	@aws lambda invoke --function-name store_value-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{ "key": "Key Object", "value": { "valString": "Sub Value 1", "valNumber": 1, "valBool": true, "valObj": { "valString": "Sub Value 2" }, "valArray": [ { "valArray": ["Sub Array 1", "Sub Array 2"] }, "some array string", 1, true ] }}'| base64) out.json | cat

test.lambda.retrieve.value:
	@aws lambda invoke --function-name retrieve_value-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo "Key Object" | base64) out.json | cat

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
