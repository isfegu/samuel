# Python Demo

This is a demo to learn how to use `dothyphen_python` from a Python application using a Wheel package.

## Usage

Create (if not already created) a python virtual environment running:

```bash
demo/python~ python3 -m venv .venv
```

Activate the python virtual environment running:

```bash
demo/python~ source .venv/bin/activate
```

Install `dothyphen_python` running:

```bash
(venv) demo/python~ pip3 install ../../dothyphen-python/output/wheels/dothyphen_python-0.2.0-cp38-abi3-linux_x86_64.whl
```

Execute the demo, running:

```bash
(venv) demo/python~ python3 demo.py
```

To close the python virtual environment, run:

```bash
(venv) demo/python~ deactivate
```
