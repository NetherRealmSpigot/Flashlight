all: lint test build

.PHONY: lint
lint:
	cargo clippy --all-features -- --deny warnings

.PHONY: lint-fix
lint-fix:
	cargo clippy --all-features --fix

.PHONY: test
test:
	cargo tarpaulin --force-clean --release --verbose --run-types AllTargets --out lcov --out stdout \
		--exclude-files src/c_varint.rs

.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build --release
	cbindgen --output target/release/flashlight.h

.SILENT: test-library-linux
test-library-linux: build
	set -e; \
	cp tests/c_varint_test.c target/release/; \
	cd target/release/; \
	rm -rf c_varint_test; \
	cc -pipe -O -Werror c_varint_test.c -I. -L. -l:./libflashlight.so -o c_varint_test; \
	chmod +x c_varint_test; \
	./c_varint_test