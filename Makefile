build:
	docker compose build

db:
	docker compose up

setup:
	cargo install sqlx-cli cargo-watch
	cargo install --path .
	# brew install --cask protopie
	# brew install protoc
	# brew install grpcurl
	cargo install cargo-edit
	cargo add tonic
	cargo add prost
	cargo add tokio --features=full
	cargo add tonic-build --build
	cargo add tonic-reflection

dev:
	sqlx db create
	sqlx migrate run
	cargo watch -x run

test:
	cargo test

test-todo:
	cargo test -- repositories::todo::test::crud_scenario

# standalone test
test-s:
	cargo test --no-default-features

run:
	cargo run

test-grpc:
	grpcurl -plaintext localhost:50051 list
	# grpcurl -plaintext localhost:50051 list hello.HelloService
	# grpcurl -plaintext -d '{"name": "World"}' localhost:50051 hello.HelloService/SayHello
