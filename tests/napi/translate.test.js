const dothyphen = require('../../dothyphen-napi/index.js');

test('can be used from nodejs', () => {
  expect(dothyphen.translate("Hello World")).toBe('.... . .-.. .-.. --- / .-- --- .-. .-.. -..');
});