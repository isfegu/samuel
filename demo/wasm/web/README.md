# WebAssembly > Web Demo

This is a demo to learn how to use `dothyphen_wasm` from a Web page.

## Requirements

Something to run a web server, like Python.

## Usage

First copy `dothyphen-wasm/output/wasm/web/dothyphen_wasm.js` and `dothyphen-wasm/output/wasm/web/dothyphen_wasm_bg.wasm` files into `demo/wasm/web/lib` directory.

Then, execute:

```bash
demo/wasm/web ~ python3 -m http.server
```

And open [http://0.0.0.0:8000/](http://0.0.0.0:8000/) in a Browser.
