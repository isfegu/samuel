# DotHyphen WebAssembly

_DotHyphen WebAssembly_ is a WebAssembly wrapper of _DotHyphen_ ready to be used as npm package.

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator developed in Rust.

## Usage

```javascript
import * as dothyphen from "@isfegu/dothyphen-wasm";

console.log(dothyphen.translate("Hello World"));  // Should print .... . .-.. .-.. --- / .-- --- .-. .-.. -..
```

## Contributing

_DotHyphen WebAssembly_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages and multiple environments.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) and the [contributing documentation](./CONTRIBUTING.md) to contribute to _DotHyphen WebAssembly_.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

All help will be appreciated.
