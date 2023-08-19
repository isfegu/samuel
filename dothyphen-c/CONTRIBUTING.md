# Contributing

_DotCyphen_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Building

> Info: The wrapper is an static library named `libdotcyphen`.

The build ouput of this crate is an static lib named `libdotcyphen.a`, moreover this crate uses [cbindgen](https://github.com/eqrion/cbindgen/blob/master/docs.md) to build the header file, `libdotcyphen.h`, needed to use the library in a C/C++ code.

To build `dothyphen-c`, run:

```bash
dothyphen-c~ cargo build
```

> The result of the build will be placed in the following locations. The static lib can be found in `target/debug/libdotcyphen.a` and the headers file can be found in `dothyphen-c/libdotcyphen.h`. This crate uses the `build.rs` file to create `libdotcyphen.h` file.

## Publishing

`dothyphen-c` it is not published anywhere, yet.
