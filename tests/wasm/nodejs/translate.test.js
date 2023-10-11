const dothyphen = require('../../../dothyphen-wasm/output/wasm/npm/dothyphen_wasm.js');

test('ascii string is translated to morse', () => {
  expect(dothyphen.to_morse("Hello World")).toBe('.... . .-.. .-.. --- / .-- --- .-. .-.. -..');
});

test('morse string is translated to ascii', () => {
  expect(dothyphen.to_ascii('.... . .-.. .-.. --- / .-- --- .-. .-.. -..')).toBe("hello world");
});