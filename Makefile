test-integration: test-integration-wasm-nodejs

test-integration-wasm-nodejs: build-wasm-npm
	cd tests/wasm/nodejs && yarn && yarn test

build-wasm-npm:
	cd dothyphen-wasm && wasm-pack build --release --target nodejs --scope isfegu --out-dir output/wasm/npm && cd ..