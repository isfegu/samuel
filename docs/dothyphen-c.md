# DotHyphen C/C++

A C/C++ wrapper of _DotHyphen_.

> Info: The wrapper is an static library named `libdotcyphen`.

## Building

The build ouput of this crate is an static lib named `libdotcyphen.a`, moreover this crate uses [cbindgen](https://github.com/eqrion/cbindgen/blob/master/docs.md) to build the header file, `libdotcyphen.h`, needed to use the library in a C/C++ code.

To build `dothyphen-c`, run:

```bash
dothyphen-c~ cargo build
```

> The result of the build will be placed in the following locations. The static lib can be found in `target/debug/libdotcyphen.a` and the headers file can be found in `dothyphen-c/libdotcyphen.h`. This crate uses the `build.rs` file to create `libdotcyphen.h` file.

## Testing

To execute the crate unit tests, run:

```bash
dothyphen-c~ cargo test
```
