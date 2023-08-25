setup:
  # brew install --cask protopie
	# brew install protoc
	# brew install grpcurl
	cargo install cargo-edit
	cargo add tonic
	cargo add prost
	cargo add tokio --features=full
	cargo add tonic-build --build
	cargo add tonic-reflection

run:
	cargo run

test-grpc:
	grpcurl -plaintext localhost:50051 list
	# grpcurl -plaintext localhost:50051 list hello.HelloService
	# grpcurl -plaintext -d '{"name": "World"}' localhost:50051 hello.HelloService/SayHello
