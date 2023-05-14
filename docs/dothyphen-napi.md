# DotHyphen Napi

A native Node.js module wrapper of _DotHyphen_.

> Info: When `dothyphen-napi` is published as Node.js module, it uses the name `@isfegu/dothyphen-napi`

## Building

This crate uses [Napi](https://napi.rs/) to make the wrapper. [Read the official documentation](https://napi.rs/docs/introduction/getting-started) to get more information about building process.

First, install the dependencies:

```bash
dothyphen-napi~ yarn # or npm install
```

To build `dothyphen-napi`, run:

```bash
dothyphen-napi~ yarn build # or npm run build
```

Following files will be created in the building process:

* `index.js`
* `index.d.ts`
* `dothyphen-napi.linux-x64-gnu.node` (_Depending on the architecture on which this crate is built, the name of the file may be different._)

## Versioning

To increase the version number of _DotHyphen Napi_, it is __mandatory__ to update the version number in `dothyphen-napi/Cargo.toml` and in `dothyphen-napi/package.json`.
