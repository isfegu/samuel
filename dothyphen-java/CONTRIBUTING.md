# Contributing

_DotJyphen_ is a part of [Samuel](https://github.com/isfegu/samuel), to contribute:

1. Read the [Samuel documentation](https://github.com/isfegu/samuel/blob/main/README.md).
2. Download the [Samuel repository](https://github.com/isfegu/samuel).
3. Install the [Samuel requirements](https://github.com/isfegu/samuel/blob/main/README.md#requirements).
4. Follow the [Samuel contributing guidelines](https://github.com/isfegu/samuel/blob/main/README.md#guidelines).

## Overview

_DotJyphen_ have two pieces. One is a dynamic library developed with Rust and the other is a Java Class that use this library and exposes a method to be used from other Java code. All this is made possible by [JNI](https://docs.oracle.com/en/java/javase/20/docs/specs/jni/intro.html).

### Library

The library is a Rust crate that uses _dothyphen_ crate to translate ASCII string to a Morse string. But the most important thing this library does is, that it can be used from Java and this is possible thanks to the [JNI crate](https://docs.rs/jni/latest/jni/).

### Java Class

The Java Class is a Java "glue" that load the library and exposes a method to be used from other any other Java code.

### FFI

All the bindings and all the "things" that we need to make Rust code able to be used from Java are provided by [JNI crate](https://docs.rs/jni/latest/jni/).

But the key regarding the communication between Rust and Java is the name used in each function developed in Rust and called from Java. The name of each function (in Rust) depends on the Java package structure, the name of the class and the name of the method (in Java) that will call Rust code.

For example, in _dothyphen-java_, the Java Class is placed in `main/java/dotjyphen/Translate.java` and the method that will call Rust code is:

```java
private static native String jniTranslate(String input);
```

Therefore, the function in Rust __must__ be named:

```rust
pub extern "system" fn Java_dotjyphen_Translate_jniTranslate<'local>
```

Each change made on the Java side must be evaluated for its impact on the Rust side.

To know what name must be used in Rust, we can run:

```bash
java/src/main/java/dotjyphen~ javac -h . Translate.java
```

This command creates the `dotjyphen_Translate.h` file. Open the file and the name needed by Rust code is there.

## Building

There are two things to build, the native library and the Java package.

### Native library

The build ouput of this crate is a dynamic lib named `libdotjyphen.so`.

```bash
dothyphen-java~ cargo build
```

> The result of the build will be placed in `target/debug/libdotjyphen.so`.

### Java package

The build ouput of this Java Class is a Java package named `dotjyphen-X.Y.Z.jar`.

Due to this Java package uses the native library to provide all the functionalities. To compile and build the Java package is mandatory to enable Java code to load the library. Please, add the path of the library to the LD_LIBRARY_PATH environment variable:

```bash
~ export LD_LIBRARY_PATH=~/Proyectos/samuel/target/debug/
```

Check that the environment variable has been set correctly:

```bash
~ echo $LD_LIBRARY_PATH
```

Now, we can compile the Java Class:

```bash
dothyphen-java/src/java~ mvn compile
```

We can run tests to check if everything works as expected (these tests are already using the library):

```bash
dothyphen-java/src/java~ mvn test
```

Next, we can build the Java package:

```bash
dothyphen-java/src/java~ mvn package
```

## Publishing

`dothyphen-java` it is not published anywhere, yet. But, it is possible to install the Java package in the Maven local repository, in this way enable other Java projects to use _DotJyphen_ at least in the local machine.
