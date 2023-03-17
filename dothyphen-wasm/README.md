# DotHyphen WebAssembly

A WebAssembly wrapper of DotHyphen.

## Building

Due to this crate uses wasm-bindgen, this crate must be built using wasm-pack. [Read the official documentation](https://rustwasm.github.io/wasm-pack/book/commands/build.html) to get more information about building process.

To build `dothyphen-wasm` to be used in a web page:

```bash
dothyphen-wasm~ wasm-pack build --release --target web --out-dir output/wasm/web
```

To build `dothyphen-wasm` to be used from a Node.js application:

```bash
dothyphen-wasm~ wasm-pack build --release --target nodejs --scope isfegu --out-dir output/wasm/npm
```

The result of the build will be placed in `output/wasm/` directory.

## Usage

Look into the demos to see how to use DotHyphen as a WebAssembly module:

* `demo`
  * `wasm`
    * `nodejs`: How to use DotHyphen from a Node.js project. [README](../demo/wasm/nodejs/README.md) file.
    * `npm`: How to use DotHyphen as a npm module. [README](../demo/wasm/npm/README.md) file.
    * `web`: How to use DotHyphen in a web page. [README](../demo/wasm/web/README.md) file.

## Publish

The result of build `dothyphen-wasm` using `nodejs` _target_ can be published to npmjs.com.

Use the [Github Action Workflow](../.github/workflows/cd.yml) to publish `dothyphen-wasm` to npmjs.com.
