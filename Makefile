build-wasm32:
	cargo build --target wasm32-unknown-unknown --release
build-linux:
	cargo build --target x86_64-unknown-linux-gnu --release
build-wasm-web:
	wasm-pack build --release --target web --out-dir output/wasm/web
build-wasm-nodejs:
	wasm-pack build --release --target nodejs --out-dir output/wasm/nodejs
build-wasm-npm:
	wasm-pack build --release --target nodejs --scope isfegu --out-dir output/wasm/npm
build-wasm: build-wasm-web build-wasm-nodejs build-wasm-npm
build: build-wasm build-wasm32 build-linux
npm-publish:
	cd output/wasm/npm && npm publish --access=public