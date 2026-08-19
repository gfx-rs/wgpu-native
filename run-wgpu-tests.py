#!/usr/bin/env python3
import re
import shutil
import subprocess
import sys
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
    text = CARGO_LOCK.read_text()
    # source = "git+URL#COMMITHASH"
    m = re.search(
        r'name = "wgpu"\n.*?\nsource = "git\+[^#"]+#([0-9a-f]{40})"', text, re.DOTALL
    )
    if not m:
        sys.exit("Could not find wgpu git commit in Cargo.lock")
    return m.group(1)


def run(*args, **kwargs):
    print(f"+ {' '.join(str(a) for a in args)}")
    subprocess.run(args, check=True, **kwargs)


def clone_at_commit(commit: str):
    if not LOCAL_WGPU.exists():
        run("git", "clone", UPSTREAM_URL, str(LOCAL_WGPU))
    else:
        run("git", "-C", str(LOCAL_WGPU), "fetch", "--all")
    run("git", "-C", str(LOCAL_WGPU), "reset", "--hard", commit)


def copy_folder_into(name):
    dest = LOCAL_WGPU / Path(name)
    print(f"Syncing {name} -> {dest}")
    if dest.exists():
        shutil.rmtree(dest)
    shutil.copytree(str(name), str(dest))


def apply_glue():
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
    new_contents = contents.replace(search_for, replace)
    with open(file, "wt", encoding="utf-8") as f:
        f.write(new_contents)


def main():
    if "--no-setup" not in sys.argv:
        commit = get_wgpu_commit()
        print(f"wgpu commit from Cargo.lock: {commit[:12]}")
        clone_at_commit(commit)
        copy_folder_into("wgpu-c-backend")
        copy_folder_into("wgpu-c-bindings")
        copy_folder_into("ffi")
        apply_glue()
        # wgpu-c-backend sits one directory deeper under .local-wgpu than it
        # does in the wgpu-native workspace, so fix its path to wgpu-native.
        # This edits our copied-in file, not wgpu, so it stays here.
        patch_file(
            ".local-wgpu/wgpu-c-bindings/Cargo.toml",
            'wgpu-native = { path = "../"',
            'wgpu-native = { path = "../../"',
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
        wgsl_errors = ".local-wgpu/naga/tests/naga/wgsl_errors.rs"
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

    if "--no-run" not in sys.argv:
        print("+ cargo run --bin wgpu-examples hello_workgroups")
        proc = subprocess.Popen(
            ("cargo", "run", "--bin", "wgpu-examples", "hello_workgroups"),
            cwd=".local-wgpu",
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
            run("cargo", "xtask", "test", "-E", exclude, cwd=".local-wgpu")
        except subprocess.CalledProcessError as e:
            sys.exit(e.returncode)
    print("Done.")


if __name__ == "__main__":
    main()
