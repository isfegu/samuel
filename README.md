# Samuel

A basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator.

The main goal is to learn how to use Rust to build a crate that can be used from other languages. Currently Samuel can be compiled to:

* WebAssembly package, to be used from Javascript/Typescript
* Python module

## Requirements

### Mandatory

* [Rust](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* [Node.js](https://nodejs.org)
* [Python3](https://www.python.org)
* [Visual Studio Code](https://code.visualstudio.com/)

## Repository structure

This project is a Cargo's [Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), composed by three members:

* `dohy`: A CLI for DotHyphen.
* `dothyphen`: A simple ASCII to Morse translator.
* `dothyphen-python`: A Python wrapper of DotHyphen.
* `dothyphen-wasm`: A WebAssembly wrapper of DotHyphen.

Moreover, you can find several demos to show how to use Samuel.

* `demo`
  * `wasm`
    * `nodejs`: How to use DotHyphen from a Node.js project.
    * `npm`: How to use DotHyphen as a npm module.
    * `web`: How to use DotHyphen in a web page.

## Contributing

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

Use integration testing to validate how Samuel works on supported targets/platforms/languages.

* Add the tests in the `tests` directory.
* Use subdirectories to group tests by target or platform or language.
* Use [`Makefile`](./Makefile) file to run the tests.

Integration tests will be executed automatically on a _pull request_ against _main_. See [`ci.yml`](./.github/workflows/ci.yml).

## Usage

To get information about how to usage Samuel, please, read the specific documentation of each crate:

* `dohy` [README](./dohy/README.md) file.
* `dothyphen` [README](./dothyphen/README.md) file.
* `dothyphen-python` [README](./dothyphen-python/README.md) file.
* `dothyphen-wasm` [README](./dothyphen-wasm/README.md) file.
