.PHONY: module
module: all
	echo '/server-bw: $${MODULE_DIR}/target/release/server-bw' > usr.manifest
	echo '/client-bw: $${MODULE_DIR}/target/release/client-bw' >> usr.manifest
	echo '/server-latency: $${MODULE_DIR}/target/release/server-latency' >> usr.manifest
	echo '/client-latency: $${MODULE_DIR}/target/release/client-latency' >> usr.manifest

all:
	(command -v cargo && cargo build --release) || echo "Please install Rust to build this app"

clean:
	(command -v cargo && cargo clean) || echo > /dev/null
	rm -f usr.manifest
