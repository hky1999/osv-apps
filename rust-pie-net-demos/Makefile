.PHONY: module
module: target/release/http-server
	echo '/http-server: $${MODULE_DIR}/target/release/http-server' > usr.manifest

target/release/http-server: src/http-server.rs
	(command -v cargo && cargo build --bin http-server --release) || echo "Please install Rust to build this app"

clean:
	(command -v cargo && cargo clean) || echo > /dev/null
	rm -f usr.manifest
