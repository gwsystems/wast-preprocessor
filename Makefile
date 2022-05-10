WASTPP := ./target/debug/wast-preprocessor

.PHONY: all
all: dist/address.wasm

dist/%.wasm: spec/test/core/%.wast
	${WASTPP} $^ dist

.PHONY: clean
clean:
	rm -f dist/*.c
	rm -f dist/*.wasm
