# Samuel

The main goal of _Samuel_ is to be an excuse to learn how to use Rust to build an utility that can be used from other languages. The utility used as excuse is _DotHyphen_, a basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator.

Currently _Samuel_ can be used from:

* Rust as a crate
* Javascript/Typescript as a WebAssembly package
* Python as a Wheel package
* C/C++ as an static lib

## Repository structure

_Samuel_ is a Cargo's [Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), composed by four members:

* `dothyphen`: A simple ASCII to Morse translator and the core of Samuel project.
* `dohy`: A CLI for _DotHyphen_.
* `dothyphen-c`: A C/C++ wrapper of _DotHyphen_.
* `dothyphen-python`: A Python wrapper of _DotHyphen_.
* `dothyphen-wasm`: A WebAssembly wrapper of _DotHyphen_.

Moreover, you can find other relevant directories, like:

* `.github`: Github actions workflows.
* `demo`: Dummy projects to show how to use _Samuel_
  * `c++`: How to use _DotHyphen_ from a C++ project.
  * `python`: How to use _DotHyphen_ from a Python project.
  * `wasm`
    * `nodejs`: How to use _DotHyphen_ from a Node.js project.
    * `npm`: How to use _DotHyphen_ as a npm module.
    * `web`: How to use _DotHyphen_ in a web page.
* `docs`: Documentation files.
* `tests`
  * `python`: Python integrations tests.
  * `wasm/nodejs`: Nodejs integrations tests.

## Contributing

### Requirements

Following dependencies must be installed to contributing _Samuel_:

* [Rust](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* [Node.js](https://nodejs.org)
* [Python3](https://www.python.org)
* [venv](https://docs.python.org/3/library/venv.html)
* [Visual Studio Code](https://code.visualstudio.com/)

### Guidelines

* Use Visual Studio Code opening the [samuel.code-workspace](./samuel.code-workspace) _workspace_ and installing all recommended extensions.
* Use [Conventional Commits](https://www.conventionalcommits.org).
* Use [Feature Branch](https://www.atlassian.com/git/tutorials/comparing-workflows/feature-branch-workflow) creating a _pull request_ to _main_.
* Use [Semantic Versioning](https://semver.org/).
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
* `dothyphen-c` [README](./docs/dothyphen-c.md) file.
* `dothyphen-python` [README](./docs/dothyphen-python.md) file.
* `dothyphen-wasm` [README](./docs/dothyphen-wasm.md) file.

## Publish

As a part of the research done in _Samuel_, all the code should be published in the appropriate language packages providers, like npmjs.com or crates.io.

Publication must be done using the [Github Action Workflow](../.github/workflows/cd.yml).

## Usage

To get information about how to usage Samuel, please, read the specific documentation of each crate:

* `dohy` [README](./dohy/README.md) file.
* `dothyphen` [README](./dothyphen/README.md) file.
* `dothyphen-c` [README](./dothyphen-c/README.md) file.
* `dothyphen-python` [README](./dothyphen-python/README.md) file.
* `dothyphen-wasm` [README](./dothyphen-wasm/README.md) file.
