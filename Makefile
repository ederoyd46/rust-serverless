BASE_DIR=$(shell pwd)
STAGE=${USER}

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

test_lambda:
	@aws lambda invoke --function-name store-$(STAGE) --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test"}' | base64) out.json | tail

test_local:
	@DATABASE=rust_serverless_store-$(STAGE) cargo run --bin store -- '{"firstName": "Test"}'
