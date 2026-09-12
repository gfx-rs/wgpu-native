# C API parity validation

The runtime additions belong to the first change; the C-backend harness and
regressions here belong to a separate dependent change. Keep production code
identical between the two branches. Test-only `cfg(test)` modules may differ.

CPU checks (Python 3.11+ and C/C++ compilers are required):

```sh
cargo test --locked -p wgpu-native --tests
cargo test --locked -p wgpu-native --no-default-features --tests
cargo test --locked -p wgpu-c-bindings --tests
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

## Metal prerequisites and known driver behavior

Shader passthrough tests need both `dxc` on PATH and the Xcode Metal Toolchain
component (`xcodebuild -downloadComponent MetalToolchain`). They generate
multiple shader representations even when testing only Metal.

Setup applies `wgpu-metal-subgroups.patch` to the pinned upstream test. macOS
27 on Apple M4 Max passes all 37 subgroup checks through both C dispatch and
direct Rust wgpu. The old mandatory Metal expected-failure rule rejected this
success. The patch permits success without broadening the three specific
accepted older-driver panic patterns or skipping any GPU assertions. Existing
prepared checkouts need the patch applied explicitly before `--no-setup`.

The same host intermittently returns zero from the written-timestamp query
test through both backends (4 failures in 10 isolated runs each). Its assertions
are unchanged; preserve failed runs as evidence, not as a C-only regression or
a passing test. `WGPU_NO_CUSTOM_BACKEND=1` runs the identical prepared test
binary through direct Rust wgpu for comparison. Do not set it for C-backend
acceptance; the runner's preflight intentionally rejects that bypass.

The generated Rust bindings preserve `C-unwind` only for the 14 runtime exports
with that ABI. Callback types remain `extern "C"`; Rust callbacks must catch
panics before returning and resume them only after the C call has completed.
The default native callbacks remain fatal. This is not permission to unwind
through an arbitrary C caller, nor a general panic-recovery contract for the
native runtime. Compile-time tests check the consumer declarations and the
32-bit `WGPUBool` polling signatures on both sides of the boundary.
