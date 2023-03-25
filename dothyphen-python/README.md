# DotHyphen Python

A Python wrapper of DotHyphen.

This wrapper is made using [PyO3](https://pyo3.rs/).

## Environment

To build `dothyphen-python` a python virtual environment (venv) is needed.

To create (if not already created) a python virtual environment, run:

```bash
dothyphen-python~ python3 -m venv .venv
```

To activate the python virtual environment, run:

```bash
dothyphen-python~ source .venv/bin/activate
```

To install the dependencies (see [requirements.txt](./requirements.txt) file), run:

```bash
(venv) dothyphen-python~ pip3 install -r requirements.txt
```

To close the python virtual environment, run:

```bash
(venv) dothyphen-python~ deactivate
```

## Building

Due to this crate uses [PyO3](https://pyo3.rs/), this crate must be built using maturin. [Read the official documentation](https://www.maturin.rs/) to get more information about building process.

To build `dothyphen-python` as module in the python virtual environment, run:

```bash
(venv) dothyphen-python~ maturin develop
```

To build `dothyphen-python` as _wheel_ package, run:

```bash
(venv) dothyphen-python~ maturin build --out output/wheels --release
```

## Testing

To run integrations tests, run:

```bash
~ make test-integration-python
```

## Usage

If the `dothyphen-python` has been built as module in the python virtual environment, it can be used from python REPL, first:

```bash
(venv) dothyphen-python~ python3
```

and then:

```python
>>> import dotpyphen
>>> dotpyphen.translate("Hello World")
'.... . .-.. .-.. --- / .-- --- .-. .-.. -..'
```

> Info: The python module name for `dothyphen-python` is `dotpyphen`.

Look into the demos to see how to use DotHyphen as a Python module:

* `demo`
  * `python`: How to use DotHyphen from a Python project. [README](../demo/python/README.md) file.

## Publish

The result of build `dothyphen-python` as _wheel_ package can be published to [PyPy](https://pypi.org/).

Use the [Github Action Workflow](../.github/workflows/cd.yml) to publish `dothyphen-python` to PyPy.
