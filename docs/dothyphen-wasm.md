# DotHyphen WebAssembly

A WebAssembly wrapper of _DotHyphen_.

## Building

Due to this crate uses wasm-bindgen, this crate must be built using wasm-pack. [Read the official documentation](https://rustwasm.github.io/wasm-pack/book/commands/build.html) to get more information about building process.

To build `dothyphen-wasm` to be used in a web page:

```bash
dothyphen-wasm~ wasm-pack build --release --target web --out-dir output/wasm/web
```

> The result of the build will be placed in `output/wasm/web` directory.

To build `dothyphen-wasm` to be used from a Node.js application:

```bash
dothyphen-wasm~ wasm-pack build --release --target nodejs --scope isfegu --out-dir output/wasm/npm
```

> The result of the build will be placed in `output/wasm/npm` directory.

## Testing

To execute the crate unit tests, run:

```bash
dothyphen-wasm~ cargo test
```
