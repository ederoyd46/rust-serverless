BASE_DIR=$(shell pwd)
STAGE=${USER}
DATA_STORE_NAME=rust_serverless_store-$(STAGE)
ENDPOINT=--endpoint-url http://localhost:8000

.PHONY: deploy

build: 
	@cargo build 

build_with_features: 
	@cargo build --all-features

release: 
	@cargo build --all-features --release

package: 
	@for i in store_event store_value; \
	do \
		mkdir -p deploy/$$i; \
		cp target/release/$$i deploy/$$i/bootstrap; \
		zip -j -9 deploy/$$i.zip deploy/$$i/bootstrap; \
	done;

plan:
	@terraform plan -var stage=$(STAGE) infrastructure

deploy:
	@terraform apply -var stage=$(STAGE) -auto-approve infrastructure

remove:
	@terraform destroy -var stage=$(STAGE) -auto-approve infrastructure

test:
	@cargo test

cross.build: 
	cross build --all-features --target=x86_64-unknown-linux-gnu --release

cross.package: 
	@for i in store_event store_value; \
	do \
		mkdir -p deploy/$$i; \
		cp target/x86_64-unknown-linux-gnu/release/$$i deploy/$$i/bootstrap; \
		zip -j -9 deploy/$$i.zip deploy/$$i/bootstrap; \
	done;

cross.build.deploy: cross.build cross.package deploy

test.lambda.event:
	@aws lambda invoke --function-name store_event-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test", "lastName": "User"}' | base64) out.json | cat

test.lambda.value:
	@aws lambda invoke --function-name store_value-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{"key": "Test", "value": {"subKey": "Sub Value"}}' | base64) out.json | cat

test.local.event:
	@for i in 1; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_event -- '{"firstName": "Test '$$i'", "lastName": "User"}'; \
	done;

test.local.value:
	@for i in 1; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Object '$$i'", "value": {"valString": "Sub Value '$$i'", "valNumber": 1, "valBool": true, "valObj": {"valString":"Sub Value 1+ '$$i'"}}}'; \
	done;

	# DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Bool '$$i'", "value": true}'; \
	# DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key Number '$$i'", "value": 1}'; \
	# DATABASE=$(DATA_STORE_NAME) cargo run --bin store_value -- '{"key": "Key String '$$i'", "value": "Value '$$i'"}'; \

table.list:
	@aws dynamodb list-tables $(ENDPOINT) | cat

table.scan:
	@aws dynamodb scan --table-name $(DATA_STORE_NAME) $(ENDPOINT) | cat

table.create:
	@aws dynamodb create-table --table-name $(DATA_STORE_NAME) \
		--attribute-definitions \
			AttributeName=PK,AttributeType=S \
		--key-schema \
			AttributeName=PK,KeyType=HASH \
		--billing-mode PAY_PER_REQUEST \
		$(ENDPOINT) \
	| cat
            
