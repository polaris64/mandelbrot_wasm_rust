all:
	cargo +nightly build --target=wasm32-unknown-unknown --release
	wasm-gc target/wasm32-unknown-unknown/release/mandelbrot_wasm.wasm target/wasm32-unknown-unknown/release/mandelbrot.wasm
	cp index.html target/wasm32-unknown-unknown/release/index.html
