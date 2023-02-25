# Samuel

Un básico traductor de ASCII a [Morse](https://es.wikipedia.org/wiki/C%C3%B3digo_morse).

El objetivo no es tener un traductor a Morse completo y totalmente funcional, el objetivo de este repositorio es aprender
a construir paquetes WebAssembly o librerías nativas, etc, partiendo de código Rust.

## Requisitos

Para poder ejecutar Samuel es necesario:

* [Rust](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* Make
* Visual Studio Code

## Estructura

* `src`:
  * `bin`: CLI para usar Samuel.
  * `lib`: Código fuente de Samuel en Rust.
* `demo`
  * `wasm`
    * `nodejs`: Uso de Samuel como paquete WebAssembly utilizado desde Node.js.
    * `npm`: Uso de Samuel como paquete npm utilizado desde Node.js.
    * `web`: Uso de Samuel como paquete WebAssembly utilizado desde entorno web.

## Contribuir

* Es recomendable usar Visual Studio Code, abrir el _workspace_ [samuel.code-workspace](./samuel.code-workspace) e instalar las extensiones recomendadas.
* Deben utilizarse [Conventional Commits](https://www.conventionalcommits.org).
* Debe utlizarse [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) haciendo un _pull request_ contra _main_ para incorporar los cambios.
* Deben añadirse pruebas unitarias.

### Pruebas unitarias

Para ejecutar las pruebas unitarias:

```bash
~ cargo test
```

## Construcción

### CLI

Se puede compilar en modo `debug`:

```bash
~ cargo build --target x86_64-unknown-linux-gnu
```

También se puede compilar en modo `release`:

```bash
~ cargo build --target x86_64-unknown-linux-gnu --release
```

> Se puede cambiar `x86_64-unknown-linux-gnu` por la arquitectura que se desee para obtener el binario adecuado.

Se puede utilizar _Make_ para la compilación en modo `release` para arquitectura `x86_64-unknown-linux-gnu`:

```bash
~ make build-linux
```

El resultado de la compilación se ubicará en el directorio `target`, dentro de un subdirectorio con el nombre de la arquitectura elegida y a su vez dentro de otro subdirectorio con el nombre del modo seleccionado, por ejemplo: `target\x86_64-unknown-linux-gnu\release`.

### WebAssembly

Para compilar Samuel a WebAssembly existe un comando _Make_ que facilita la tarea:

```bash
~ make build-wasm
```

El resultado de la compilación se ubicará en el directorio `target\wasm32-unknown-unknown\release`.

Esta compilación hace uso de _wasm-pack_ para construir todo lo necesario para que Samuel pueda ser consumido desde Javascript. El resultado de la compilación para ser usado desde Javascript se ubicará en el directorio `output/wasm`. Dentro de este directorio se crearán los siguientes subdirectorios:

* `output/wasm/nodejs`: Para ser usado directamente desde una aplicación Node.js.
* `output/wasm/npm`: Para ser usado desde una aplicación Node.js utilizando un paquete _npm_.
* `output/wasm/web`: Para ser usado dede una página web.

#### Publicación en npmjs

El contenido del directorio `output/wasm/npm` es lo que debe publicarse en npmjs.com.

Antes de publicar debe hacerse _login_ mediante:

```bash
output/wasm/npm ~ npm login
```

Una vez hecho el _login_ correctamente ya se puede publicar mediante:

```bash
output/wasm/npm ~ npm publish --access=public
```

## Uso

### Cargo

Para ejecutar Samuel desde Cargo:

```bash
~ cargo run -- --translate "Hello world"
```

### CLI

```bash
~ ./target/x86_64-unknown-linux-gnu/release/samuel --translate "Hello World"
```

> Se puede ambiar _release_ por _debug_ según el modo seleccionado en la compilación o `x86_64-unknown-linux-gnu` por la arquitectura seleccionada en la compilación.

### WebAssembly

Para mostrar cómo usar Samuel como paquete WebAssembly es mejor ver las siguientes demos:

* `demo/wasm/nodejs`: Aplicación Node.js que utiliza Samuel directamente. [Ver documentación].
* `demo/wasm/npm`: Aplicación Node.js que utiliza Samuel como paquete _npm_.
* `demo/wasm/web`: Página web que utiliza Samuel.
