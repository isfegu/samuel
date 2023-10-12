#!/bin/bash 
command="wasmtime ../../../target/wasm32-wasi/release/dothyphen-wasi.wasm -- --translate '.... . .-.. .-.. --- / .-- --- .-. .-.. -..' --output ascii"
eval "$command"
exit $?