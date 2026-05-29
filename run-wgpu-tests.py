#!/usr/bin/env python3
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

REPO_URL = "https://github.com/inner-daemons/wgpu.git"
LOCAL_WGPU = Path(".local-wgpu")
WGPU_C_BACKEND_SRC = Path("wgpu-c-backend")
CARGO_LOCK = Path("Cargo.lock")

patches = """
# PATCH
[patch."https://github.com/inner-daemons/wgpu.git"]
wgpu-core = { path = "./wgpu-core" }
wgpu-hal = { path = "./wgpu-hal" }
wgpu-types = { path = "./wgpu-types" }
naga = { path = "./naga" }
"""


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
        run("git", "clone", REPO_URL, str(LOCAL_WGPU))
    else:
        run("git", "-C", str(LOCAL_WGPU), "fetch", "--all")
    run("git", "-C", str(LOCAL_WGPU), "reset", "--hard", commit)


def copy_c_backend():
    dest = LOCAL_WGPU / WGPU_C_BACKEND_SRC.name
    if dest.exists():
        print(f"Removing existing {dest}")
        shutil.rmtree(dest)
    print(f"Copying {WGPU_C_BACKEND_SRC} -> {dest}")
    shutil.copytree(str(WGPU_C_BACKEND_SRC), str(dest))


def patch_file(file, search_for, replace):
    with open(file, "rt", encoding="utf-8") as f:
        contents = f.read()
    new_contents = contents.replace(search_for, replace)
    with open(file, "wt", encoding="utf-8") as f:
        f.write(new_contents)


def append_file(file, contents):
    with open(file, "at", encoding="utf-8") as f:
        f.write(contents)


def patch_add_c_backend():
    c_backend_dir = (LOCAL_WGPU / "wgpu-c-backend").resolve()

    for cargo_toml in LOCAL_WGPU.rglob("Cargo.toml"):
        if any(part == "target" for part in cargo_toml.parts):
            continue
        if cargo_toml.parent.resolve() == c_backend_dir:
            continue

        content = cargo_toml.read_text(encoding="utf-8")
        if not re.search(r"(?m)^wgpu\s*[=.{]", content):
            continue

        rel_path = os.path.relpath(c_backend_dir, cargo_toml.parent.resolve()).replace(
            "\\", "/"
        )
        dep_line = f'wgpu-c-backend = {{ path = "{rel_path}" }}\n'

        if "wgpu-c-backend" not in content:
            if "[dependencies]\n" in content:
                content = content.replace(
                    "[dependencies]\n", "[dependencies]\n" + dep_line, 1
                )
            else:
                content += f"\n[dependencies]\n{dep_line}"
            cargo_toml.write_text(content, encoding="utf-8")
            print(f"  + wgpu-c-backend dep -> {cargo_toml}")

        for rs_name in ("main.rs", "lib.rs"):
            rs_path = cargo_toml.parent / "src" / rs_name
            if not rs_path.exists():
                continue
            rs = rs_path.read_text(encoding="utf-8")
            if "extern crate wgpu_c_backend" not in rs:
                lines = rs.splitlines(keepends=True)
                # Inner attributes (#![...]) must precede all items and may
                # span multiple lines. Track bracket depth to find each
                # attribute's end, then insert after the last complete one.
                insert_at = 0
                i = 0
                while i < len(lines):
                    stripped = lines[i].strip()
                    if stripped.startswith("#!["):
                        depth = 0
                        while i < len(lines):
                            depth += lines[i].count("[") - lines[i].count("]")
                            i += 1
                            if depth <= 0:
                                insert_at = i
                                break
                    elif stripped.startswith("//!"):
                        i += 1
                        insert_at = i
                    elif not stripped or stripped.startswith("//"):
                        i += 1
                    else:
                        break
                lines.insert(insert_at, "extern crate wgpu_c_backend;\n")
                rs_path.write_text("".join(lines), encoding="utf-8")
                print(f"  + extern crate wgpu_c_backend -> {rs_path}")


def main():
    if "--no-setup" not in sys.argv:
        commit = get_wgpu_commit()
        print(f"wgpu commit from Cargo.lock: {commit[:12]}")
        clone_at_commit(commit)
        copy_c_backend()
        patch_file(
            ".local-wgpu/Cargo.toml", "members = [\n", 'members = ["wgpu-c-backend",\n'
        )
        patch_file(
            ".local-wgpu/Cargo.toml",
            "default-members = [\n",
            'default-members = ["wgpu-c-backend",\n',
        )
        patch_file(
            ".local-wgpu/wgpu-c-backend/Cargo.toml",
            'wgpu-native = { path = "../"',
            'wgpu-native = { path = "../../"',
        )
        append_file(".local-wgpu/Cargo.toml", patches)
        patch_add_c_backend()

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
        try:
            run("cargo", "xtask", "test", cwd=".local-wgpu")
        except subprocess.CalledProcessError as e:
            sys.exit(e.returncode)
    print("Done.")


if __name__ == "__main__":
    main()
