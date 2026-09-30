#!/usr/bin/env python3
"""Run public C API surface-lifetime regressions on an actual macOS Metal device."""

import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

CASES = [
    "present-retained-frame", "discard-retained-frame", "reconfigure-retained-frame",
    "threaded-old-frame-release", "unconfigure-retained-frame", "release-surface-first",
    "unconfigure-acquired-frame", "destroy-without-present", "present-destroyed-frame",
    "discard-destroyed-frame", "drop-without-present", "discard-status",
]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native-source", type=Path, required=True)
    parser.add_argument("--library", type=Path,
                        help="Use an existing library for diagnostics instead of building the runtime")
    parser.add_argument("--target-dir", type=Path, help="Cargo target directory for the runtime build")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--case", choices=CASES, action="append")
    args = parser.parse_args()
    if sys.platform != "darwin":
        parser.error("this runner requires macOS and a real Metal adapter")
    source, output = (p.resolve() for p in (args.native_source, args.output))
    output.mkdir(parents=True, exist_ok=True)
    if args.library:
        library = args.library.resolve()
    else:
        target = (args.target_dir or output / "cargo-target").resolve()
        subprocess.run(["cargo", "build", "--locked", "--lib", "--manifest-path", str(source / "Cargo.toml"),
                        "--target-dir", str(target)], check=True)
        library = target / "debug/libwgpu_native.a"
    with library.open("rb") as stream:
        library_hash = hashlib.file_digest(stream, "sha256").hexdigest()
    revision = subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip()
    diff = subprocess.check_output(["git", "-C", str(source), "diff", "HEAD"])
    executable = output / "metal-surface-lifetime"
    subprocess.run([
        "xcrun", "clang++", "-std=c++17", "-fobjc-arc", "-g", "-Wall", "-Wextra", "-Werror",
        str(Path(__file__).with_name("metal_surface_lifetime.mm")),
        "-I", str(source / "ffi"), "-I", str(source / "ffi/webgpu-headers"),
        str(library), "-framework", "AppKit", "-framework", "Metal", "-framework", "QuartzCore",
        "-framework", "CoreGraphics", "-framework", "IOKit", "-framework", "IOSurface",
        "-o", str(executable),
    ], check=True)
    results = []
    for case in args.case or CASES:
        try:
            run = subprocess.run([str(executable), case], capture_output=True, text=True, timeout=60,
                                 env={**os.environ, "DYLD_LIBRARY_PATH": str(library.parent)})
            log = run.stdout + run.stderr
            passed = run.returncode == 0 and f"PASS {case}" in log
            result = {"case": case, "returncode": run.returncode, "passed": passed}
        except subprocess.TimeoutExpired as error:
            log = str(error)
            result = {"case": case, "passed": False, "timeout": True}
        (output / f"{case}.log").write_text(log)
        results.append(result)
        print(f"{case}: {'PASS' if result['passed'] else 'FAIL'}", flush=True)
    (output / "results.json").write_text(json.dumps({
        "nativeSource": str(source), "revision": revision,
        "sourceDiffSha256": hashlib.sha256(diff).hexdigest(),
        "library": str(library), "librarySha256": library_hash,
        "prebuiltLibrary": args.library is not None, "results": results,
    }, indent=2) + "\n")
    return 0 if all(result["passed"] for result in results) else 1


if __name__ == "__main__":
    sys.exit(main())
