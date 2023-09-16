wasm:
	cargo build --target wasm32-unknown-unknown
	rm -Rf generated/
	wasm-bindgen --target web --out-dir generated/ ../../target/wasm32-unknown-unknown/debug/gamepads_wasm_bindgen.wasm

serve-wasm: wasm
	python3 -m http.server 9000
