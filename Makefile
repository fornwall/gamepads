PACKAGE_NAME = gamepads
MODE =
WASM_DIR = debug
WASM_OPT = wasm-opt --all-features --disable-gc
ifeq ($(RELEASE),1)
  MODE = --release
  WASM_DIR = release
  WASM_OPT += -O3
else
  WASM_OPT += -O1
endif

CLIPPY_PARAMS = -- \
	-W clippy::cargo \
	-W clippy::cast_lossless \
	-W clippy::dbg_macro \
	-W clippy::expect_used \
	-W clippy::if_not_else \
	-W clippy::items_after_statements \
	-W clippy::large_stack_arrays \
	-W clippy::linkedlist \
	-W clippy::manual_filter_map \
	-W clippy::match_same_arms \
	-W clippy::needless_continue \
	-W clippy::needless_pass_by_value \
	-W clippy::nursery \
	-W clippy::option_if_let_else \
	-W clippy::print_stderr \
	-W clippy::print_stdout \
	-W clippy::redundant_closure_for_method_calls \
	-W clippy::semicolon_if_nothing_returned \
	-W clippy::similar_names \
	-W clippy::single_match_else \
	-W clippy::trivially_copy_pass_by_ref \
	-W clippy::unnested_or_patterns \
	-W clippy::unreadable-literal \
	-W clippy::unseparated-literal-suffix \
	-A clippy::needless-doctest-main \
	-D warnings \
	-A clippy::multiple_crate_versions

check:
	cargo fmt --check
	cargo clippy $(CLIPPY_PARAMS) --no-deps
	cargo clippy --target aarch64-linux-android --all-features $(CLIPPY_PARAMS) --no-deps
	cargo clippy --target wasm32-unknown-unknown $(CLIPPY_PARAMS) --no-deps
	cargo clippy --target wasm32-unknown-unknown --all-features $(CLIPPY_PARAMS) --no-deps
	cd examples/hello-gamepads && make
	cd examples/gamepads-macroquad && make
	cargo test

run:
	cargo run $(MODE)

wasm:
	cargo build --target wasm32-unknown-unknown  $(MODE)
	$(WASM_OPT) -o $(PACKAGE_NAME).wasm ./target/wasm32-unknown-unknown/$(WASM_DIR)/$(PACKAGE_NAME).wasm
	cargo build --target wasm32-unknown-unknown --features wasm-bindgen $(MODE)
	$(WASM_OPT) -o $(PACKAGE_NAME).wasm ./target/wasm32-unknown-unknown/$(WASM_DIR)/$(PACKAGE_NAME).wasm

serve-wasm: wasm
	python3 -m http.server 9000

generate-js:
	cd js && ./generate-js.sh

check-js: generate-js
	cd js && git diff --exit-code .

clean:
	cargo clean

.PHONY: check run wasm serve-wasm clean generate-js check-js
