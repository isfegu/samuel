# Samuel

The main goal of _Samuel_ is to be an excuse to learn how to use Rust to build a utility that can be used from other languages. The utility used as excuse is _DotHyphen_, a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator.

Currently _Samuel_ can be used from:

* Rust as a crate
* Javascript/Typescript as a
  * WebAssembly Node.js module
  * Native Node.js module
* Python as a Wheel package
* C/C++ as an static lib
* <https://dothyphen.fermyon.app/> as an HTTP EndPoint

## Repository structure

_Samuel_ is a Cargo's [Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), composed by four members:

* `dohy`: A CLI for _DotHyphen_.
* `dothyphen`: A simple ASCII to Morse translator and the core of Samuel project.
* `dothyphen-c`: A C/C++ wrapper of _DotHyphen_.
* `dothyphen-fermyon`: A rust wrapper of _DotHyphen_ to enable Fermyon Cloud deployment.
* `dothyphen-napi`: A native Node.js module of _DotHyphen_.
* `dothyphen-python`: A Python wrapper of _DotHyphen_.
* `dothyphen-wasm`: A WebAssembly wrapper of _DotHyphen_.

Moreover, you can find other relevant directories, like:

* `.github`: Github actions workflows.
* `demo`: Dummy projects to show how to use _Samuel_
  * `c++`: How to use _DotHyphen_ from a C++ project.
  * `napi`: How to use _DotHyphen_ from a Node.js project using a native Node.js module.
  * `python`: How to use _DotHyphen_ from a Python project.
  * `wasm`
    * `nodejs`: How to use _DotHyphen_ from a Node.js project.
    * `npm`: How to use _DotHyphen_ from a Node.js project using a WebAssembly Node.js module.
    * `web`: How to use _DotHyphen_ in a web page.
* `docs`: Documentation files.
* `tests`
  * `c++`: C++ integration tests.
  * `napi`: Node.js native module integration tests.
  * `python`: Python integrations tests.
  * `wasm/nodejs`: Node.js WebAssembly module integrations tests.

## Contributing

### Requirements

Following dependencies must be installed to contributing _Samuel_:

* [Rust](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* [`wasm32-wasi` target](https://rust-lang.github.io/rustup/concepts/toolchains.html)
* [Node.js](https://nodejs.org)
* [Yarm](https://yarnpkg.com/) or Npm (already installed with Node.js)
* [Python3](https://www.python.org)
* [venv](https://docs.python.org/3/library/venv.html)
* [Spin](https://developer.fermyon.com/spin/install)

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

### Build

Read build information about each _Samuel_'s crates.

* `dohy` [README](./docs/dohy.md) file.
* `dothyphen` [README](./docs/dothyphen.md) file.
* `dothyphen-fermyon` [README](./docs/dothyphen-fermyon.md) file.
* `dothyphen-c` [README](./docs/dothyphen-c.md) file.
* `dothyphen-python` [README](./docs/dothyphen-python.md) file.
* `dothyphen-wasm` [README](./docs/dothyphen-wasm.md) file.

## Publish

As a part of the research done in _Samuel_, all the code should be published in the appropriate language packages or cloud providers, like npmjs.com, crates.io or Fermyon Cloud.

Publication must be done using the [Github Action Workflow](../.github/workflows/cd.yml).

## Usage

To get information about how to usage Samuel, please, read the specific documentation of each crate:

* `dohy` [README](./dohy/README.md) file.
* `dothyphen` [README](./dothyphen/README.md) file.
* `dothyphen-fermyon` [README](./dothyphen-fermyon/README.md) file.
* `dothyphen-c` [README](./dothyphen-c/README.md) file.
* `dothyphen-python` [README](./dothyphen-python/README.md) file.
* `dothyphen-wasm` [README](./dothyphen-wasm/README.md) file.
