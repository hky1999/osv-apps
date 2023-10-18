.PHONY: module
module: all
	echo '/allocate: $${MODULE_DIR}/target/release/allocate' > usr.manifest
	echo '/allocate_zero: $${MODULE_DIR}/target/release/allocate_zero' >> usr.manifest
	echo '/thread_spawn: $${MODULE_DIR}/target/release/thread_spawn' >> usr.manifest
	echo '/thread_yield: $${MODULE_DIR}/target/release/thread_yield' >> usr.manifest

all:
	(command -v cargo && cargo build --release) || echo "Please install Rust to build this app"

clean:
	(command -v cargo && cargo clean) || echo > /dev/null
	rm -f usr.manifest
