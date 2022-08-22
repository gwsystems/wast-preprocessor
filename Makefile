CC := clang

WASTPP := ./target/debug/wast-preprocessor
RUNTIME_PATH=../awsm/runtime
RUNTIME_INCLUDES=-I${RUNTIME_PATH} -I${RUNTIME_PATH}/libc/wasi/include -I${RUNTIME_PATH}/thirdparty/dist/include

WASM_CPATH+=${RUNTIME_PATH}/runtime.c
WASM_CPATH+=${RUNTIME_PATH}/libc/env.c
WASM_CPATH+=${RUNTIME_PATH}/memory/no_protection.c

.PHONY: dist/%.wasm
dist/%.wasm: spec/test/core/%.wast
	${WASTPP} $^ dist

dist/%.wat: dist/%.wasm
	wasm2wat -o $@ $^

dist/%.bc: dist/%.wasm
	../awsm/target/debug/awsm -o $@ $^

dist/%.ll: dist/%.bc
	llvm-dis-12 -o $@ $^

dist/%.awsm: dist/%.bc dist/%.c ${WASM_CPATH}
	${CC} -pthread -ldl -lm -O0 -flto -g3 ${RUNTIME_INCLUDES} $^ -o $@

.PHONY: clean
clean:
	@rm -f dist/*.c
	@rm -f dist/*.wasm
	@rm -f dist/*.wat
	@rm -f dist/*.awsm
	@rm -f dist/*.ll
	@rm -f dist/*.bc

# FIXME: Error here
test_address: dist/address.wasm dist/address_0.awsm dist/address_1.awsm dist/address_2.awsm dist/address_3.awsm
	@echo -n "Address 0: "
	@./dist/address_0.awsm && echo "Success"
	@echo -n "Address 1: "
	@./dist/address_1.awsm && echo "Success"
	@echo -n "Address 2: "
	@./dist/address_2.awsm && echo "Success"
	@echo -n "Address 3: "
	@./dist/address_3.awsm && echo "Success"

test_align: dist/align.wasm

# We seem to not support binary encoded WAST
# test_binary-leb128: dist/binary-leb128.wasm

# We seem to not support binary encoded WAST
# test_binary: dist/binary.wasm

# Requires multi-return values
test_block: dist/block.wasm dist/block_0.awsm
	@echo -n "Block 0: "
	@./dist/block_0.awsm && echo "Success"

# Uses Exports with invalid C indentifers
test_br_if: dist/br_if.wasm dist/br_if_0.awsm
	@echo -n "Br_if 0: "
	@./dist/br_if_0.awsm && echo "Success"

# Requires reference types support
test_br_table: dist/br_table.wasm

# Uses Exports with invalid C indentifers
test_br: dist/br.wasm dist/br_if_0.awsm
	@echo -n "Br 0: "
	@./dist/br_0.awsm && echo "Success"

# Requires bulk memory instruction support
test_bulk: dist/bulk.wasm dist/bulk_0.awsm dist/bulk_1.awsm dist/bulk_2.awsm dist/bulk_3.awsm dist/bulk_4.awsm dist/bulk_5.awsm dist/bulk_6.awsm
	@echo -n "Br 0: "
	@./dist/br_0.awsm && echo "Success"
	@echo -n "Br 1: "
	@./dist/br_1.awsm && echo "Success"
	@echo -n "Br 2: "
	@./dist/br_2.awsm && echo "Success"
	@echo -n "Br 3: "
	@./dist/br_3.awsm && echo "Success"
	@echo -n "Br 4: "
	@./dist/br_4.awsm && echo "Success"
	@echo -n "Br 5: "
	@./dist/br_5.awsm && echo "Success"
	@echo -n "Br 6: "
	@./dist/br_6.awsm && echo "Success"

# Requires multi-return values
test_call_indirect: dist/call_indirect.wasm dist/call_indirect_0.awsm dist/call_indirect_1.awsm
	@echo -n "call_indirect 0: "
	@./dist/call_indirect_0.awsm && echo "Success"
	@echo -n "call_indirect 1: "
	@./dist/call_indirect_1.awsm && echo "Success"

# Requires multi-return values
test_call: dist/call.wasm dist/call_0.awsm
	@echo -n "call 0: "
	@./dist/call_0.awsm && echo "Success"

# Just generates an empty module
test_comments: dist/comments.wasm

# TODO: Generates 299 wasm modules. Need some Make-fu to compile and run all these automatically 
test_const:  dist/const.wasm 

# 'not implemented: I32TruncSatF32S'
test_conversions:  dist/conversions.wasm  dist/conversions_0.awsm
	@echo -n "conversions 0: "
	@./dist/conversions_0.awsm && echo "Success"

test_custom: dist/custom.wasm

# FIXME: Overwrites modules... Why?
test_data: dist/data.wasm

# Uses dashes in exports
test_elem: dist/elem.wasm dist/elem_0.awsm dist/elem_1.awsm dist/elem_2.awsm dist/elem_3.awsm dist/elem_4.awsm dist/elem_5.awsm dist/elem_6.awsm
	@echo -n "elem 0: "
	@./dist/elem_0.awsm && echo "Success"
	@echo -n "elem 1: "
	@./dist/elem_1.awsm && echo "Success"
	@echo -n "elem 2: "
	@./dist/elem_2.awsm && echo "Success"
	@echo -n "elem 3: "
	@./dist/elem_3.awsm && echo "Success"
	@echo -n "elem 4: "
	@./dist/elem_4.awsm && echo "Success"
	@echo -n "elem 5: "
	@./dist/elem_5.awsm && echo "Success"
	@echo -n "elem 6: "
	@./dist/elem_6.awsm && echo "Success"

# FIXME: Invalid parameter declaration
test_endianness: dist/endianness.wasm dist/endianness_0.awsm
	@echo -n "endianness 0: "
	@./dist/endianness_0.awsm && echo "Success"

# FIXME: Invalid parameter declaration
test_exports: dist/exports.wasm dist/exports_0.awsm dist/exports_1.awsm dist/exports_2.awsm dist/exports_3.awsm
	@echo -n "exports 0: "
	@./dist/exports_0.awsm && echo "Success"
	@echo -n "exports 1: "
	@./dist/exports_1.awsm && echo "Success"
	@echo -n "exports 2: "
	@./dist/exports_2.awsm && echo "Success"
	@echo -n "exports 3: "
	@./dist/exports_3.awsm && echo "Success"

# FIXME: Invalid parameter declaration
test_f32_bitwise: dist/f32_bitwise.wasm dist/f32_bitwise_0.awsm
	@echo -n "f32_bitwise 0: "
	@./dist/f32_bitwise_0.awsm && echo "Success"

# FIXME: Invalid parameter declaration
test_f32_cmp: dist/f32_cmp.wasm dist/f32_cmp_0.awsm
	@echo -n "f32_cmp 0: "
	@./dist/f32_cmp_0.awsm && echo "Success"

# FIXME: Invalid parameter declaration
test_f32: dist/f32.wasm dist/f32_0.awsm
	@echo -n "f32 0: "
	@./dist/f32_0.awsm && echo "Success"

test_f64_bitwise: dist/f64_bitwise.wasm dist/f64_bitwise_0.awsm
	@echo -n "f64_bitwise 0: "
	@./dist/f64_bitwise_0.awsm && echo "Success"

test_f64_cmp: dist/f64_cmp.wasm dist/f64_cmp_0.awsm
	@echo -n "f64_cmp 0: "
	@./dist/f64_cmp_0.awsm && echo "Success"

# dist/f64.wasm
# dist/fac.wasm
# dist/float_exprs.wasm
# dist/float_literals.wasm
# dist/float_memory.wasm
# dist/float_misc.wasm
# dist/forward.wasm
# dist/func_ptrs.wasm
# dist/func.wasm
# dist/global.wasm
# dist/i32.wasm
# dist/i64.wasm
# dist/if.wasm
# dist/imports.wasm
# dist/inline-module.wasm
# dist/int_exprs.wasm
# dist/int_literals.wasm
# dist/labels.wasm
# dist/left-to-right.wasm
# dist/linking.wasm
# dist/load.wasm
# dist/local_get.wasm
# dist/local_set.wasm
# dist/local_tee.wasm
# dist/loop.wasm
# dist/memory_copy.wasm
# dist/memory_fill.wasm
# dist/memory_grow.wasm
# dist/memory_init.wasm
# dist/memory_redundancy.wasm
# dist/memory_size.wasm
# dist/memory_trap.wasm
# dist/memory.wasm
# dist/names.wasm
# dist/nop.wasm
# dist/ref_func.wasm
# dist/ref_is_null.wasm
# dist/ref_null.wasm
# dist/return.wasm
# dist/select.wasm
# dist/skip-stack-guard-page.wasm
# dist/stack.wasm
# dist/start.wasm
# dist/store.wasm
# dist/switch.wasm
# dist/table_copy.wasm
# dist/table_fill.wasm
# dist/table_get.wasm
# dist/table_grow.wasm
# dist/table_init.wasm
# dist/table_set.wasm
# dist/table_size.wasm
# dist/table-sub.wasm
# dist/table.wasm
# dist/token.wasm
# dist/traps.wasm
# dist/type.wasm
# dist/unreachable.wasm
# dist/unreached-invalid.wasm
# dist/unreached-valid.wasm
# dist/unwind.wasm
# dist/utf8-custom-section-id.wasm
# dist/utf8-import-field.wasm
# dist/utf8-import-module.wasm
# dist/utf8-invalid-encoding.wasm