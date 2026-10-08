#!/usr/bin/env python3
"""Check native usage extensions through the C API on a Metal adapter with ray queries and texture atomics."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native-source", type=Path, required=True)
    parser.add_argument("--target-dir", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if sys.platform != "darwin":
        parser.error("this runner requires macOS and a real Metal adapter")
    source, target, output = (p.resolve() for p in (args.native_source, args.target_dir, args.output))
    output.mkdir(parents=True, exist_ok=True)
    subprocess.run(["cargo", "build", "--locked", "--lib", "--manifest-path", str(source / "Cargo.toml"),
                    "--target-dir", str(target)], check=True)
    library = target / "debug/libwgpu_native.a"
    executable = output / "native-usage"
    subprocess.run([
        "xcrun", "clang++", "-std=c++17", "-Wall", "-Wextra", "-Werror",
        str(Path(__file__).with_name("native_usage.cpp")),
        "-I", str(source / "ffi"), "-I", str(source / "ffi/webgpu-headers"),
        str(library), "-framework", "Metal", "-framework", "QuartzCore",
        "-framework", "CoreGraphics", "-framework", "IOKit", "-framework", "IOSurface",
        "-framework", "Foundation", "-o", str(executable),
    ], check=True)
    run = subprocess.run([str(executable)], capture_output=True, text=True, timeout=60,
                         env={**os.environ, "DYLD_LIBRARY_PATH": str(library.parent)})
    log = run.stdout + run.stderr
    (output / "native-usage.log").write_text(log)
    print(log, end="")
    passed = run.returncode == 0 and "PASS native-usage" in log
    with library.open("rb") as stream:
        library_hash = hashlib.file_digest(stream, "sha256").hexdigest()
    revision = subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip()
    diff = subprocess.check_output(["git", "-C", str(source), "diff", "HEAD"])
    (output / "results.json").write_text(json.dumps({
        "nativeSource": str(source), "revision": revision,
        "sourceDiffSha256": hashlib.sha256(diff).hexdigest(),
        "library": str(library), "librarySha256": library_hash,
        "testSourceSha256": hashlib.sha256(Path(__file__).with_name("native_usage.cpp").read_bytes()).hexdigest(),
        "returncode": run.returncode, "passed": passed,
    }, indent=2) + "\n")
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
