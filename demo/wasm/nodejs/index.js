import { to_ascii, to_morse } from '../../../dothyphen-wasm/output/wasm/npm/dothyphen_wasm.js';

console.log(to_morse("Hello World"));
console.log(to_ascii(".... . .-.. .-.. --- / ... .- -- ..- . .-.."));