# DotHyphen Shuttle

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator developed in Rust. _DotHyphen Shuttle_ provides a way to use _DotHyphen_ as a [HTTP API from Shuttle Cloud](https://dothyphen.shuttleapp.rs).

## Usage

### Translation

`URL https://dothyphen-shuttle.shuttleapp.rs`

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
  --url https://dothyphen-shuttle.shuttleapp.rs/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```
