all: test

.PHONY: test
test:
	cargo tarpaulin --force-clean --release --verbose --run-types AllTargets --out lcov --out stdout

.PHONY: clean
clean:
	cargo clean
