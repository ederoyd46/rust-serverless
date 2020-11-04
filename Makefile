BASE_DIR=$(shell pwd)
STAGE=${USER}
DATA_STORE_NAME=rust_serverless_store-$(STAGE)

.PHONY: deploy

build: 
	@cargo build 

build_with_features: 
	@cargo build --all-features

release: 
	@cargo build --all-features --release

package: 
	@mkdir -p deploy/store
	@cp target/release/store deploy/store/bootstrap
	@zip -j -9 deploy/store.zip deploy/store/bootstrap

plan:
	@terraform plan -var stage=$(STAGE) infrastructure

deploy:
	@terraform apply -var stage=$(STAGE) -auto-approve infrastructure

remove:
	@terraform destroy -var stage=$(STAGE) -auto-approve infrastructure

test:
	@cargo test

cross_build: 
	cross build --all-features --target=x86_64-unknown-linux-gnu --release

cross_package: 
	@mkdir -p deploy/store
	@cp target/x86_64-unknown-linux-gnu/release/store deploy/store/bootstrap
	@zip -j deploy/store.zip deploy/store/bootstrap

cross_build_deploy: cross_build cross_package deploy

lambda_test:
	@aws lambda invoke --function-name store-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test", "lastName": "User"}' | base64) out.json | cat

local_test:
	@for i in =1 2 3 4; \
	do \
		DATABASE=$(DATA_STORE_NAME) cargo run --bin store -- '{"firstName": "Test '$$i'", "lastName": "User"}'; \
	done;
local_tables:
	@aws dynamodb list-tables --endpoint-url http://localhost:8000 | cat

local_table_scan:
	@aws dynamodb scan --table-name $(DATA_STORE_NAME) --endpoint-url http://localhost:8000 | cat

local_table_create:
	@aws dynamodb create-table --table-name $(DATA_STORE_NAME) \
		--attribute-definitions \
			AttributeName=firstName,AttributeType=S \
		--key-schema \
			AttributeName=firstName,KeyType=HASH \
		--billing-mode PAY_PER_REQUEST \
		--endpoint-url http://localhost:8000 \
	| cat
            


