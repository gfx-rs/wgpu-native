# C API parity validation

The runtime additions belong to the first change; the C-backend harness and
regressions here belong to a separate dependent change. Keep production code
identical between the two branches. Test-only `cfg(test)` modules may differ.

CPU checks (Python 3.11+ and C/C++ compilers are required):

```sh
cargo test --locked -p wgpu-native --tests
cargo test --locked -p wgpu-native --no-default-features --tests
python tests/check_headers.py
python -m unittest discover -s tests -p 'test_*.py'
```

Run the upstream wgpu suite through the first change's runtime checkout:

```sh
python run-wgpu-tests.py --native-source /absolute/path/to/feature-checkout
```

The runner verifies the prepared bindings resolve to that runtime. Use
`--no-setup` to reuse a prepared harness, `--no-run` to prepare only, or
`--work-dir` for an independent wgpu checkout. Setup refuses to overwrite a
modified checkout. `--offline` requires the pinned wgpu and integration-glue
commits to already exist locally, plus cached Cargo dependencies.

The `hello_workgroups` preflight must succeed and log that it is using the
C backend before the suite runs. A missing GPU adapter is a blocked run, not
a passing or skipped suite. The wasm dependency-graph and instance-report
lifetime tests are explicitly excluded because the C adapter cannot implement
those test mechanisms yet; this is not full upstream test-suite coverage.

Header defaults can also be checked directly against the first checkout:

```sh
python tests/check_headers.py --native-source /absolute/path/to/feature-checkout
```
