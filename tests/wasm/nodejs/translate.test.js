const dothyphen = require('../../../dothyphen-wasm/output/wasm/npm/dothyphen_wasm.js');

test('can be used from nodejs', () => {
  expect(dothyphen.translate("Hello World")).toBe('.... . .-.. .-.. --- / .-- --- .-. .-.. -..');
});