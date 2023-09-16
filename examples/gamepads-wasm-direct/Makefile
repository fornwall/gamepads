wasm:
	cargo build --target wasm32-unknown-unknown
	cp ../../target/wasm32-unknown-unknown/debug/gamepads_wasm_direct.wasm .

serve-wasm: wasm
	python3 -m http.server 9000
