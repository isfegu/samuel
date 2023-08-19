# DotHyphen Fermyon

_DotHyphen Fermyon_ provides a way to use _DotHyphen_ as a [HTTP API from Fermyon Cloud](https://dothyphen.fermyon.app/translate).

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator developed in Rust.

## Usage

### Translation

`URL https://dothyphen.fermyon.app`

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

The Morse code string.

```json
{
  "output":".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
}
```

#### Example

```bash
curl --request POST \
  --url https://dothyphen.fermyon.app/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```

## Contributing

_DotHyphen Fermyon_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages and multiple environments.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) and the [contributing documentation](./CONTRIBUTING.md) to contribute to _DotHyphen Fermyon_.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

All help will be appreciated.
