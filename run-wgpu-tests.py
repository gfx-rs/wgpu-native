#!/usr/bin/env python3
import argparse
import json
import os
import shutil
import subprocess
import sys
import tomllib
from pathlib import Path

UPSTREAM_URL = "https://github.com/gfx-rs/wgpu.git"
FORK_URL = "https://github.com/inner-daemons/wgpu.git"
# Branch on the fork that carries the wgpu-side integration glue as a single
# commit (see apply_glue). Replaces the string-patching this script used to do.
GLUE_BRANCH = "c-backend-testrig-v31"
# Exact commit of GLUE_BRANCH to apply. Pinned (rather than tracking the branch
# tip) so a later force-push to the branch can't silently change the glue.
# Update this when intentionally moving to a new glue commit; the pinned commit
# must be authored on top of the wgpu revision pinned in Cargo.lock.
GLUE_COMMIT = "30fdffc30f2b87633d74b21ff9df641d54801204"
LOCAL_WGPU = Path(".local-wgpu")
CARGO_LOCK = Path("Cargo.lock")


def get_wgpu_commit() -> str:
    packages = tomllib.loads(CARGO_LOCK.read_text())["package"]
    matches = [p for p in packages if p["name"] == "wgpu" and p.get("source", "").startswith("git+")]
    if len(matches) != 1:
        sys.exit("Could not find wgpu git commit in Cargo.lock")
    commit = matches[0]["source"].rsplit("#", 1)[-1]
    if len(commit) != 40 or any(c not in "0123456789abcdef" for c in commit):
        sys.exit("Expected a full wgpu commit hash in Cargo.lock")
    return commit


def run(*args, **kwargs):
    print(f"+ {' '.join(str(a) for a in args)}")
    subprocess.run(args, check=True, **kwargs)


def clone_at_commit(commit: str, offline: bool):
    if not LOCAL_WGPU.exists():
        if offline:
            sys.exit("Offline setup requires an existing clean wgpu checkout")
        run("git", "clone", "--no-checkout", UPSTREAM_URL, str(LOCAL_WGPU))
    elif subprocess.check_output(["git", "-C", str(LOCAL_WGPU), "status", "--porcelain"]).strip():
        sys.exit("Refusing to overwrite a modified wgpu checkout; use --no-setup or a fresh --work-dir")
    if not offline:
        run("git", "-C", str(LOCAL_WGPU), "fetch", "origin", commit)
    run("git", "-C", str(LOCAL_WGPU), "checkout", "--detach", commit)


def copy_folder_into(name):
    dest = LOCAL_WGPU / Path(name)
    print(f"Syncing {name} -> {dest}")
    if dest.exists():
        shutil.rmtree(dest)
    shutil.copytree(str(name), str(dest))


def apply_glue(offline: bool):
    """Apply the wgpu-side integration glue on top of whatever commit is
    currently checked out.

    The glue — adding wgpu-c-backend to the workspace, redirecting the
    inner-daemons/wgpu.git git dep to local paths, and injecting a
    wgpu-c-backend dep + `extern crate wgpu_c_backend;` into every crate that
    depends on wgpu — lives as a single commit (GLUE_COMMIT on GLUE_BRANCH) in
    the fork. We fetch that exact commit and apply just its diff, so the glue
    tracks whatever wgpu commit Cargo.lock pins without being reproduced here.

    wgpu-c-backend source itself is NOT in that commit; it stays
    source-of-truth in this repo and is copied in by copy_c_backend().
    """
    print(f"Applying glue {GLUE_COMMIT[:12]} ({GLUE_BRANCH})")
    if not offline:
        run("git", "-C", str(LOCAL_WGPU), "fetch", "--quiet", FORK_URL, GLUE_COMMIT)
    diff = subprocess.run(
        ["git", "-C", str(LOCAL_WGPU), "diff", f"{GLUE_COMMIT}^", GLUE_COMMIT],
        check=True,
        stdout=subprocess.PIPE,
    ).stdout
    # --3way falls back to a real merge if context has drifted from the glue's
    # base commit, and fails loudly on a true conflict (regenerate the branch).
    subprocess.run(
        ["git", "-C", str(LOCAL_WGPU), "apply", "--3way"],
        check=True,
        input=diff,
    )


def patch_file(file, search_for, replace):
    with open(file, "rt", encoding="utf-8") as f:
        contents = f.read()
    if contents.count(search_for) != 1:
        sys.exit(f"Expected exactly one patch location in {file}; source has drifted")
    new_contents = contents.replace(search_for, replace)
    with open(file, "wt", encoding="utf-8") as f:
        f.write(new_contents)


def main():
    global LOCAL_WGPU
    parser = argparse.ArgumentParser(description="Run wgpu tests through a selected wgpu-native runtime checkout (Python 3.11+).")
    parser.add_argument("--no-setup", action="store_true")
    parser.add_argument("--no-run", action="store_true")
    parser.add_argument("--offline", action="store_true")
    parser.add_argument("--work-dir", type=Path, default=LOCAL_WGPU)
    parser.add_argument("--native-source", type=Path, help="Feature-only wgpu-native checkout; defaults to this repository")
    args = parser.parse_args()
    LOCAL_WGPU = args.work_dir.resolve()
    native_source = (args.native_source or Path(__file__).resolve().parent).resolve()
    os.chdir(Path(__file__).resolve().parent)
    if args.offline:
        os.environ["CARGO_NET_OFFLINE"] = "true"
    if not (native_source / "Cargo.toml").is_file():
        sys.exit(f"Missing runtime Cargo.toml: {native_source}")
    if not args.no_setup:
        commit = get_wgpu_commit()
        print(f"wgpu commit from Cargo.lock: {commit[:12]}")
        clone_at_commit(commit, args.offline)
        copy_folder_into("wgpu-c-backend")
        copy_folder_into("wgpu-c-bindings")
        copy_folder_into("ffi")
        apply_glue(args.offline)
        # wgpu-c-backend sits one directory deeper under .local-wgpu than it
        # does in the wgpu-native workspace, so fix its path to wgpu-native.
        # This edits our copied-in file, not wgpu, so it stays here.
        patch_file(
            LOCAL_WGPU / "wgpu-c-bindings/Cargo.toml",
            'wgpu-native = { path = "../"',
            'wgpu-native = { path = ' + json.dumps(str(native_source)),
        )
        # naga's recursion_depth_template drives the WGSL parser to its
        # depth-200 recursion guard, which needs ~2 MiB of stack in debug
        # builds (~10 KiB per tracked recursion, measured on macOS arm64;
        # Windows x64 frames are larger still). nextest runs one process per
        # test, so the test runs on the process main thread, and the 1 MiB
        # (MSVC) / 2 MiB (GNU) Windows main-thread stacks die with
        # STATUS_STACK_OVERFLOW before the guard can fire. Run the check on an
        # explicitly sized thread instead — the same fix wgsl_errors.rs already
        # applies to limit_braced_statement_nesting — so the guard fires and
        # its error stays covered on every platform. Lowering the guard itself
        # is not an option: it must stay above the brace-nesting limit (127),
        # or limit_braced_statement_nesting and too_many_unclosed_loops report
        # the recursion error instead of their own.
        wgsl_errors = LOCAL_WGPU / "naga/tests/naga/wgsl_errors.rs"
        patch_file(
            wgsl_errors,
            '''fn recursion_depth_template() {
    check(
        include_str!("deep-template.wgsl"),
        r#"error: internal WGSL front end error
 = note: Parser recursion limit exceeded

"#,
    );
}''',
            '''fn recursion_depth_template() {
    // Depth 200 needs more stack than a Windows main thread has in debug
    // builds; use an explicitly sized thread like limit_braced_statement_nesting.
    std::thread::Builder::new()
        .stack_size(1024 * 1024 * 4)
        .spawn(|| {
            check(
                include_str!("deep-template.wgsl"),
                r#"error: internal WGSL front end error
 = note: Parser recursion limit exceeded

"#,
            );
        })
        .unwrap()
        .join()
        .unwrap()
}''',
        )
        with open(wgsl_errors, "rt", encoding="utf-8") as f:
            if "stack_size(1024 * 1024 * 4)" not in f.read():
                sys.exit(
                    "ERROR: failed to move recursion_depth_template onto a sized "
                    "thread; the test source has drifted — update the patch in "
                    "run-wgpu-tests.py (or drop it if upstream fixed the test)."
                )

    manifest_path = LOCAL_WGPU / "wgpu-c-bindings/Cargo.toml"
    manifest = tomllib.loads(manifest_path.read_text())
    configured_source = (manifest_path.parent / manifest["dependencies"]["wgpu-native"]["path"]).resolve()
    if configured_source != native_source:
        sys.exit(f"Prepared harness targets {configured_source}, not requested runtime {native_source}")
    print(f"Testing runtime: {native_source}")
    if not args.no_run:
        print("+ cargo run --bin wgpu-examples hello_workgroups")
        proc = subprocess.Popen(
            ("cargo", "run", "--bin", "wgpu-examples", "hello_workgroups"),
            cwd=LOCAL_WGPU,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
        )
        lines = []
        for line in proc.stdout:
            print(line, end="", flush=True)
            lines.append(line)
        proc.wait()
        combined = "".join(lines)
        needle = "Creating instance through wgpu-c-backend"
        if needle not in combined:
            sys.exit(f"ERROR: expected '{needle}' in hello_workgroups output")
        if proc.returncode != 0:
            sys.exit(f"ERROR: hello_workgroups exited with code {proc.returncode}")
        # Tests that can't pass through the C backend, excluded from the run:
        # - wgpu-dependency: asserts the wasm dep graph; injecting wgpu-c-backend
        #   (which pulls in wgpu-native -> wgpu-core -> naga) inherently breaks it.
        # - device_lifetime_check / mem_leaks: assert on `Instance::generate_report()`,
        #   which the C backend doesn't implement (returns None -> `.unwrap()` panics).
        exclude = (
            "not ("
            "binary(wgpu-dependency)"
            " or test(device_lifetime_check)"
            " or test(/mem_leaks::/)"
            ")"
        )
        try:
            run("cargo", "xtask", "test", "-E", exclude, cwd=LOCAL_WGPU)
        except subprocess.CalledProcessError as e:
            sys.exit(e.returncode)
    print("Done.")


if __name__ == "__main__":
    main()
