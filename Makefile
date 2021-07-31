all: server

server:
	cargo run --bin helloworld-server
	
client:
	cargo run --bin helloworld-client

server1:
	cargo run --bin routeguide-server
	
client1:
	cargo run --bin routeguide-client