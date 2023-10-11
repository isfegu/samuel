# WebAssembly > Web Demo

This is a demo to learn how to use `dothyphen_wasm` from a Web page.

## Requirements

Something to run a web server, like Python.

## Usage

First build `dothyphen_wasm` to be used in a web page, following the instructions located in the [CONTRIBUTING.md](../../../dothyphen-wasm/CONTRIBUTING.md) file.

Next, copy `dothyphen-wasm/output/wasm/web/dothyphen_wasm.js` and `dothyphen-wasm/output/wasm/web/dothyphen_wasm_bg.wasm` files into `demo/wasm/web/lib` directory.

Then, execute:

```bash
demo/wasm/web ~ python3 -m http.server
```

And open [http://0.0.0.0:8000/](http://0.0.0.0:8000/) in a Browser.
