BASE_DIR=$(shell pwd)

.PHONY: deploy

build: 
	@cargo build 

release: 
	@cargo build --all-features --release

package: 
	@mkdir -p deploy/store
	@cp target/release/store deploy/store/bootstrap
	@zip -j -9 deploy/store.zip deploy/store/bootstrap

plan:
	@terraform plan

deploy:
	@terraform apply -auto-approve

test:
	@cargo test

cross: 
	cross build --all-features --target=x86_64-unknown-linux-gnu --release

cross_package: 
	@mkdir -p deploy/store
	@cp target/x86_64-unknown-linux-gnu/release/store deploy/store/bootstrap
	@zip -j deploy/store.zip deploy/store/bootstrap



test_lambda:
	@aws lambda invoke --function-name Store --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test"}' | base64) out.json | tail

test_local:
	@DATABASE=rust_serverless_store cargo run --bin store -- '{"firstName": "Test"}'
