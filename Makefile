BASE_DIR=$(shell pwd)

.PHONY: deploy

build: 
	cargo build --target=x86_64-unknown-linux-gnu

package: build
	@mkdir -p deploy/store
	@mkdir -p deploy/retrieve

	@cp target/release/store deploy/store/bootstrap
	@cp target/release/retrieve deploy/retrieve/bootstrap
	@zip -j deploy/store.zip deploy/store/bootstrap
	@zip -j deploy/retrieve.zip deploy/retrieve/bootstrap

plan:
	@terraform plan

deploy:
	@terraform apply -auto-approve

# sam-deploy:
# 	sam package --template-file template.yml --s3-bucket matt-sam-deployments --output-template-file ready.yaml
# 	sam deploy --template-file ready.yaml --stack-name HelloRust --capabilities CAPABILITY_IAM


