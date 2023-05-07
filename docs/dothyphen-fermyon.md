# DotHyphen Fermyon

A rust wrapper of _DotHyphen_ to enable Fermyon Cloud deployment.

> Info: This wrapper is served from Fermyon Cloud as WebAssembly.

## Environment

Following dependencies are mandatory:

* The [Spin](https://developer.fermyon.com/spin) framework.
* The `wasm32-wasi` target for Rust.

Read the [official documentation to install Spin](https://developer.fermyon.com/spin/install).

> Info: In this documentation we assume Spin is installed inside `dothyphen-fermyon` directory.

## Building

To build `dothyphen-fermyon`, run:

```bash
dothyphen-fermyon~ ./spin build
```

## Contributing

We can run _DotHyphen Fermyon_ locally simulating the Fermyon Cloud environment, running:

```bash
dothyphen-fermyon~ ./spin up
```

This command starts a local server at <http://127.0.0.1:3000/translate>. Any `println!` call will show the output in the console once a request is done to the EndPoint.

`curl` or applications like _Imsommia_ can be used to do the requests.

```bash
curl --request POST \
  --url http://127.0.0.1:3000/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```

Read the [Fermyon documentation](https://developer.fermyon.com/spin/rust-components) about how to use Rust to write Fermyon applications.

_DotHyphen Fermyon_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) to contribute to _DotHyphen Fermyon_. All help will be appreciated.

## Usage

### Translation

`URL http://127.0.0.1:3000`

#### Endpoint

`POST /translate`

Request must be a JSON content.

#### Request

`input` _mandatory_

The ASCII string to translate to Morse code.

```json
{
  "input": "Hello World"
}
```

#### Response

`output`

the Morse code string.

```json
{
  "output":".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
}
```

#### Example

```bash
curl --request POST \
  --url http://127.0.0.1:3000/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```
