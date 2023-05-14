#[macro_use]
extern crate napi_derive;

#[napi]
pub fn translate(input: String) -> String {
    dothyphen::translate(&input)
}
