# C++ Demo

This is a demo to learn how to use `dothyphen_c` from a C++ code.

This demo uses `gcc` and `Make` to compile the source code. Therefore, if you want to run this demo you __must__ install both.

> This demo has been tested only in GNU/Linux.

## Usage

Bompile `dothyphen_c` running:

```bash
dothyphen-c~ cargo build
```

The static library compiled will be placed in `target/debug/libdotcyphen.a`. The header file created will be placed in `dothyphen_c/libdotcyphen.h`.

Compile the demo source code:

```bash
demo/c++~ make build
```

Execute the demo:

```bash
demo/c++~ ./demo
```
