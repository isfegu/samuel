# DotHyphen Python

A Python wrapper of _DotHyphen_.

> Info: The python module name for `dothyphen-python` is `dotpyphen`.

## Environment

To build `dothyphen-python` a python virtual environment, [venv](https://docs.python.org/3/library/venv.html), is needed.

To create (if not already created) a Python virtual environment, run:

```bash
dothyphen-python~ python3 -m venv .venv
```

To activate the Python virtual environment, run:

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

Due to this crate uses [PyO3](https://pyo3.rs/) to make the bindings, this crate must be built using maturin. [Read the official documentation](https://www.maturin.rs/) to get more information about building process.

To build `dothyphen-python` to be automatically added to the project Python virtual environment, run:

```bash
(venv) dothyphen-python~ maturin develop
```

Now it can be used from the Python REPL:

```bash
(venv) dothyphen-python~ python3
```

and then:

```python
>>> import dotpyphen
>>> dotpyphen.translate("Hello World")
```

To build `dothyphen-python` as _wheel_ package, run:

```bash
(venv) dothyphen-python~ maturin build --out output/wheels --release
```

Now, the _wheel_ package can be installed, globally or in any Python virtual environment using pip:

```bash
pip3 install ../../dothyphen-python/output/wheels/dothyphen_python-0.1.0-cp38-abi3-linux_x86_64.whl
```

## Testing

To execute the crate unit tests, run:

```bash
dothyphen-python~ cargo test
```
