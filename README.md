# Samuel

A basic ASCII to [Morse](https://en.wikipedia.org/wiki/Morse_code) translator.

The main goal is to learn how to build WebAssembly packages and native libraries or modules using Rust.

## Requirements

### Mandatory

* [Rust](https://rustup.rs/)
* [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* [Node.js](https://nodejs.org)
* [Visual Studio Code](https://code.visualstudio.com/)

## Repository structure

This project is a Cargo's [Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html), composed by three members:

* `dohy`: A CLI for DotHyphen.
* `dothyphen`: A simple ASCII to Morse translator.
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
* Add unit testing whenever possible.

### Unit testing

To execute the _Workspace_ unit tests, run:

```bash
~ cargo test
```

## Usage

To get information about how to usage Samuel, please, read the specific documentation of each crate:

* `dohy` [README](./dohy/README.md) file.
* `dothyphen` [README](./dothyphen/README.md) file.
* `dothyphen-wasm` [README](./dothyphen-wasm/README.md) file.
