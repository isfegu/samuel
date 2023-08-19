# Contributing

_DotHyphen WebAssembly_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Building

> Info: When `dothyphen-wasm` is published as Node.js module, it uses the name `@isfegu/dothyphen-wasm`

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

## Publishing

Read the `dothyphen-wasm` [RELEASE](./RELEASE.md) file.
