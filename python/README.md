# dq-prof (PyPI wrapper)

Python wrapper that bundles the prebuilt `dq-prof` binary and exposes it as a `dq-prof` console script.

## Build steps
1) Build the Rust binary: `./release.sh`
2) Copy into the package: `./python/prepare_binary.sh`
3) Build wheel: `cd python && python -m build`

Upload the produced wheel to PyPI. Users can then `pip install dq-prof` and run `dq-prof ...` without installing Rust.
