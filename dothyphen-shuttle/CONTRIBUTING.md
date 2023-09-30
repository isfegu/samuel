# Contributing

_DotHyphen Shuttle_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Building

To build `dothyphen-shuttle`, run:

```bash
dothyphen-shuttle~ cargo build
```

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

## Publishing

Read the `dothyphen-shuttle` [RELEASE](./RELEASE.md) file.
