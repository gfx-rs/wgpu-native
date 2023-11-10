# Python examples

These examples use the wgpu-native DLL using `cffi`. It makes it
relatively easy to write examples and tests. Everything is deliberately
pretty raw, so the code does not look very Pythonic, but this way there
is no magic.

To run these examples, you need a Python 3 interpreter, and have a recent version of `cffi` installed (e.g. using `pip install -U cffi`).

Note: running the code from within Visual Studio Code is fragile, it looks like Code's debugger does not play well with the ffi or something.
