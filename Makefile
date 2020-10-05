BASE_DIR=$(shell pwd)

.PHONY: deploy

build: 
	cargo build 

cross: 
	cross build --target=x86_64-unknown-linux-gnu --release

package: cross
	@mkdir -p deploy/store
	@mkdir -p deploy/retrieve

	@cp target/x86_64-unknown-linux-gnu/release/store deploy/store/bootstrap
	@cp target/x86_64-unknown-linux-gnu/release/retrieve deploy/retrieve/bootstrap
	@zip -j deploy/store.zip deploy/store/bootstrap
	@zip -j deploy/retrieve.zip deploy/retrieve/bootstrap

plan:
	@terraform plan

deploy:
	@terraform apply -auto-approve

test:
	@aws lambda invoke --function-name Store --invocation-type=RequestResponse --payload $(shell echo '{"firstName": "Test"}' | base64) out


# sam-deploy:
# 	sam package --template-file template.yml --s3-bucket matt-sam-deployments --output-template-file ready.yaml
# 	sam deploy --template-file ready.yaml --stack-name HelloRust --capabilities CAPABILITY_IAM


