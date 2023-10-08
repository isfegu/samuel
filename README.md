# Samuel

The main goal of _Samuel_ is to be an excuse to learn how to use Rust to build a utility that can be used from other languages and multiple environments. The utility used as excuse is _DotHyphen_, a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) and vice versa translator.

Currently _Samuel_ can be used from:

* __Rust__ as a crate.
* __Javascript/Typescript__ as a:
  * WebAssembly Node.js module.
  * Native Node.js module.
* __Python__ as a Wheel package.
* __C/C++__ as an static lib.
* __Java__ as a dynamic lib called using JNI and jar package.
* <https://dothyphen.fermyon.app/> as an HTTP EndPoint.
* <https://dothyphen-shuttle.shuttleapp.rs> as an HTTP EndPoint.

## Repository structure

_Samuel_ is a Cargo's [Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), composed by the following members:

* `dohy`: A CLI for _DotHyphen_.
* `dothyphen`: A simple ASCII to Morse and vice versa translator and the core of Samuel project.
* `dothyphen-c`: A C/C++ wrapper of _DotHyphen_.
* `dothyphen-fermyon`: A rust wrapper of _DotHyphen_ to enable Fermyon Cloud deployment.
* `dothyphen-java`: A Java wrapper of _DotHyphen_.
* `dothyphen-napi`: A native Node.js module of _DotHyphen_.
* `dothyphen-python`: A Python wrapper of _DotHyphen_.
* `dothyphen-shuttle`: A rust wrapper of _DotHyphen_ to enable Shuttle Cloud deployment.
* `dothyphen-wasm`: A WebAssembly wrapper of _DotHyphen_.

Moreover, you can find other relevant directories, like:

* `.github`: Github actions workflows.
* `demo`: Dummy projects to show how to use _Samuel_.
  * `c++`: How to use _DotHyphen_ from a C++ project.
  * `java`: How to use _DotHyphen_ from a Java project.
  * `napi`: How to use _DotHyphen_ from a Node.js project using a native Node.js module.
  * `python`: How to use _DotHyphen_ from a Python project.
  * `wasm`
    * `nodejs`: How to use _DotHyphen_ from a Node.js project.
    * `npm`: How to use _DotHyphen_ from a Node.js project using a WebAssembly Node.js module.
    * `web`: How to use _DotHyphen_ in a web page.
* `tests`
  * `c++`: C++ integration tests.
  * `napi`: Node.js native module integration tests.
  * `python`: Python integrations tests.
  * `wasm/nodejs`: Node.js WebAssembly module integrations tests.

## Contributing

To get information about how to contribute to _Samuel_, please, read the specific documentation of each Cargo's Workspace member:

* `dohy` [CONTRIBUTING](./dohy/CONTRIBUTING.md) file.
* `dothyphen` [CONTRIBUTING](./dothyphen/CONTRIBUTING.md) file.
* `dothyphen-c` [CONTRIBUTING](./dothyphen-c/CONTRIBUTING.md) file.
* `dothyphen-java` [CONTRIBUTING](./dothyphen-java/CONTRIBUTING.md) file.
* `dothyphen-fermyon` [CONTRIBUTING](./dothyphen-fermyon/CONTRIBUTING.md) file.
* `dothyphen-napi` [CONTRIBUTING](./dothyphen-napi/CONTRIBUTING.md) file.
* `dothyphen-python` [CONTRIBUTING](./dothyphen-python/CONTRIBUTING.md) file.
* `dothyphen-shuttle` [CONTRIBUTING](./dothyphen-shuttle/CONTRIBUTING.md) file.
* `dothyphen-wasm` [CONTRIBUTING](./dothyphen-wasm/CONTRIBUTING.md) file.

Moreover, you can use the _Samuel_ [Github Discussions](https://github.com/isfegu/samuel/discussions).

### Requirements

The following requirements must be installed to contribute to each _Samuel_ Cargo's Workspace member:

#### Common

* Last stable Rust toolchain. Use [Rustup](https://rustup.rs/) to install it.

#### dothyphen-c

* [gcc](https://gcc.gnu.org/) or some C compiler.

#### dothyphen-java

* [maven](https://maven.apache.org/).
* [Open JDK](https://openjdk.org/) or [JDK](https://www.oracle.com/java/technologies/downloads/)

#### dothyphen-fermyon

* The [Spin](https://developer.fermyon.com/spin) CLI.
* [`wasm32-wasi` target](https://rust-lang.github.io/rustup/concepts/toolchains.html).

Read the [official documentation](https://developer.fermyon.com/spin/install) to install Spin CLI.

> Info: In this documentation we assume Spin is installed inside `dothyphen-fermyon` directory.

#### dothyphen-napi

* [Node.js](https://nodejs.org).
* [Yarm](https://yarnpkg.com/) or Npm (already installed with Node.js).

#### dothyphen-python

* [Python3](https://www.python.org).
* [venv](https://docs.python.org/3/library/venv.html).

#### dothyphen-shuttle

* The [Shuttle](https://docs.shuttle.rs) CLI.

Read the [official documentation](https://docs.shuttle.rs/introduction/installation) to install Shuttle CLI.

> Info: In this documentation we assume Shuttle CLI is installed globally.

#### dothyphen-wasm

* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).
* [`wasm32-wasi` target](https://rust-lang.github.io/rustup/concepts/toolchains.html).
* [Node.js](https://nodejs.org).
* [Yarm](https://yarnpkg.com/) or Npm (already installed with Node.js).

### Guidelines

* If you use Visual Studio Code, open the [samuel.code-workspace](./samuel.code-workspace) _workspace_ and install all recommended extensions.
* Use [Conventional Commits](https://www.conventionalcommits.org).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a _pull request_ to _main_.
* Use [Semantic Versioning](https://semver.org/). Each change done in any crate __MUST__ update the crate's version. Each crate have its own version number.
* Add unit and integration testing whenever possible.

### Unit testing

To execute the _Workspace_ unit tests, run:

```bash
~ cargo test
```

Unit tests will be executed automatically on a _pull request_ against _main_. See [`ci.yml`](./.github/workflows/ci.yml).

### Integration testing

Use integration testing to validate if _Samuel_ works on supported targets or platforms or languages.

* Add the tests in the `tests` directory.
* Use subdirectories to group tests by target or platform or language.
* Use [`Makefile`](./Makefile) file to run the tests.

Integration tests will be executed automatically on a _pull request_ against _main_. See [`ci.yml`](./.github/workflows/ci.yml).

### Publish

As a part of the research done in _Samuel_, all the code should be published in the appropriate language packages or cloud providers, like npmjs.com, crates.io, Fermyon Cloud or Shuttle Cloud.

Publication must be done using the [Github Action Workflow](../.github/workflows/cd.yml).

#### Versioning and Tags

All code updates of any Cargo's Workspace member must have a version number. This number can be different between members. Each version must be associated with a git tag therefore this tags will be used to publishing. These tags must be placed in the right commit and must be named using this pattern, `vX.Y.Z-<member>`, for example:

* `v0.1.0-dothyphen`
* `v0.1.1-dothyphen-c`
* `v0.2.0-dohy`

## Usage

To get information about how to usage _Samuel_, please, read the specific documentation of each _Samuel_ Cargo's Workspace member:

* `dohy` [README](./dohy/README.md) file.
* `dothyphen` [README](./dothyphen/README.md) file.
* `dothyphen-c` [README](./dothyphen-c/README.md) file.
* `dothyphen-java` [README](./dothyphen-java/README.md) file.
* `dothyphen-fermyon` [README](./dothyphen-fermyon/README.md) file.
* `dothyphen-napi` [README](./dothyphen-napi/README.md) file.
* `dothyphen-python` [README](./dothyphen-python/README.md) file.
* `dothyphen-shuttle` [README](./dothyphen-shuttle/README.md) file.
* `dothyphen-wasm` [README](./dothyphen-wasm/README.md) file.
