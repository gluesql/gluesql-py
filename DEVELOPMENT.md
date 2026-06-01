## 🚴 Usage

### Build

To build `gluesql-py`, run below command.

```
maturin build
```

And to install the built package, run below command.

```
pip install --force-reinstall target/wheels/*.whl
```

### Test

To run `gluesql-py` tests, run below command.

```
pytest
```

## Deployment

To build `gluesql-py` in release mode, run below command.

```
maturin build --release --strip
```

To deploy `gluesql-py` in [pypi](https://pypi.org/), run below command.

```
maturin publish
```
