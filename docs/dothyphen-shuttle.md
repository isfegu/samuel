# DotHyphen Shuttle

A rust wrapper of _DotHyphen_ to enable Shuttle Cloud deployment.

> Info: This wrapper is served from Shuttle Cloud as a Rust binary.

## Environment

Following dependencies are mandatory:

* The [Shuttle](https://docs.shuttle.rs) CLI.

Read the [official documentation to install Shuttle CLI](https://docs.shuttle.rs/introduction/installation).

> Info: In this documentation we assume Shuttle CLI is installed globally.

## Building

To build `dothyphen-shuttle`, run:

```bash
dothyphen-shuttle~ cargo build
```

> Important: Unlike other _Workspace members_, _DotHyphen Shuttle_ uses _DotHyphen_ from crates.io instead of `dothyphen` folder/member. This is because when _DotHyphen Shuttle_ is built from Shuttle Cloud only `dothyphen-shuttle` member is available but not others like `dothyphen`, therefore `dothyphen` dependency is obtained from crates.io (as you can see in [Cargo.toml](../dothyphen-shuttle/Cargo.toml) file).

## Contributing

We can run _DotHyphen Shuttle_ locally simulating the Shuttle Cloud environment, running:

```bash
dothyphen-shuttle~ cargo shuttle run
```

This command starts a local server at <http://127.0.0.1:8000/translate>. Any `println!` call will show the output in the console once a request is done to the EndPoint.

`curl` or applications like _Imsommia_ can be used to do the requests.

```bash
curl --request POST \
  --url http://127.0.0.1:8000/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```

Read the [Shuttle documentation](https://docs.shuttle.rs/introduction/welcome) about how to use Rust to write Shuttle applications.

_DotHyphen Shuttle_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) to contribute to _DotHyphen Shuttle_. All help will be appreciated.

## Usage

### Translation

`URL http://127.0.0.1:8000`

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
  --url http://127.0.0.1:8000/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```
