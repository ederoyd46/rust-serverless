BASE_DIR=$(shell pwd)

.PHONY: deploy

build: 
	cargo build 

cross: 
	cross build --all-features --target=x86_64-unknown-linux-gnu --release

package: cross package_store

package_store:
	@mkdir -p deploy/store
	@cp target/x86_64-unknown-linux-gnu/release/store deploy/store/bootstrap
	@zip -j deploy/store.zip deploy/store/bootstrap

package_retrieve:
	@mkdir -p deploy/retrieve
	@cp target/x86_64-unknown-linux-gnu/release/retrieve deploy/retrieve/bootstrap
	@zip -j deploy/retrieve.zip deploy/retrieve/bootstrap

plan:
	@terraform plan

deploy:
	@terraform apply -auto-approve

test:
	@aws lambda invoke --function-name Store --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test2"}' | base64) out.json | tail

test_local:
	@DATABASE=rust_serverless_store cargo run -- '{"firstName": "Test2"}'
# sam-deploy:
# 	sam package --template-file template.yml --s3-bucket matt-sam-deployments --output-template-file ready.yaml
# 	sam deploy --template-file ready.yaml --stack-name HelloRust --capabilities CAPABILITY_IAM


