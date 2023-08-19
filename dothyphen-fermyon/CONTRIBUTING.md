# Contributing

_DotHyphen Fermyon_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Building

> Info: This wrapper is served from Fermyon Cloud as WebAssembly.

To build `dothyphen-fermyon`, run:

```bash
dothyphen-fermyon~ ./spin build
```

We can run _DotHyphen Fermyon_ locally simulating the Fermyon Cloud environment, running:

```bash
dothyphen-fermyon~ ./spin up
```

This command starts a local server at <http://127.0.0.1:3000/translate>. Any `println!` call will show the output in the console once a request is done to the EndPoint.

`curl` or applications like _[Imsommia](https://insomnia.rest)_ can be used to do the requests.

```bash
curl --request POST \
  --url http://127.0.0.1:3000/translate \
  --header 'Content-Type: application/json' \
  --data '{"input": "Hello World"}'
```

Read the [Fermyon documentation](https://developer.fermyon.com/spin/rust-components) about how to use Rust to write Fermyon applications.

## Publishing

Read the `dothyphen-fermyon` [RELEASE](./RELEASE.md) file.
