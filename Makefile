all: server

server:
	cargo run --bin helloworld-server
	
client:
	cargo run --bin helloworld-client