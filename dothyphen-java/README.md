# DotJyphen

_DotJyphen_ is a Java wrapper of _DotHyphen_.

_DotHyphen_ is a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator developed in Rust.

## Usage

```java
import dotjyphen.Translate;

public class Demo {
  private static Translate translate = new Translate();

  public static void main(String[] args)  {
    System.out.println(translate.intoMorse("Hello World"));
  }
}
```

To use _DotJyphen_, you need:

* `dotjyphen-X.Y.Z.jar`: Replace X.Y.Z with the last version of _DotJyphen_.
* `libdotjyphen.so`: Replace `.so` with the right extension depending on the Operating System.
* To add the path to `libdotjyphen.so` library in the `LD_LIBRARY_PATH` environment variable.

Look at the [demo](../demo/java/) to see how to use _DotJyphen_ from Java.

## Contributing

_DotJyphen_ is a part of [Samuel](https://github.com/isfegu/samuel), a project focused on learning how to use Rust to build a crate that can be used from other languages and multiple environments.

Please read the [contributing guidelines](https://github.com/isfegu/samuel#contributing) and the [contributing documentation](./CONTRIBUTING.md) to contribute to _DotJyphen_.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

All help will be appreciated.
