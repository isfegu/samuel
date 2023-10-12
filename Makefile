test-integration: test-integration-c test-integration-napi test-integration-python test-integration-wasm-nodejs test-integration-wasm-wasi

test-integration-c: build-c-staticlib
	cd tests/c++; \
	make test; \
	./tests

test-integration-napi: build-napi-module
	cd tests/napi; \
	yarn; \
	yarn test

test-integration-python: build-python-wheel
	cd tests/python; \
	python3 -m venv .venv; \
	. .venv/bin/activate; \
	pip3 install ../../dothyphen-python/output/wheels/dothyphen_python-0.2.0-cp38-abi3-linux_x86_64.whl; \
	python3 -m unittest -v tests.test_dotpyphen; \
	deactivate

test-integration-wasm-nodejs: build-wasm-npm
	cd tests/wasm/nodejs; \
	yarn; \
	yarn test

test-integration-wasm-wasi: build-wasm-wasi
	cd tests/wasm/wasi; \
	./wasmtime.test.sh

build-c-staticlib:
	cd dothyphen-c; \
	cargo build; \
	cd ..

 build-napi-module:
	cd dothyphen-napi; \
	yarn; \
	yarn build

build-python-wheel:
	cd dothyphen-python; \
	python3 -m venv .venv; \
	. .venv/bin/activate; \
	pip3 install -r requirements.txt; \
	maturin build --out output/wheels --release --interpreter 3.8 --compatibility linux; \
	deactivate; \
	cd ..

build-wasm-npm:
	cd dothyphen-wasm; \
	wasm-pack build --release --target nodejs --scope isfegu --out-dir output/wasm/npm; \
	cd ..

build-wasm-wasi:
	cd dothyphen-wasi; \
	cargo build --target wasm32-wasi --release; \
	cd ..
