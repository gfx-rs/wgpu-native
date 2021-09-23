"""
A script to run some integrity checks on the code.
"""

import os
import sys


# %% Utilities


def remove_c_comments(code):
    """Remove C-style comments from the given code."""
    pos = 0
    new_code = ""

    while True:
        # Find start of comment
        lookfor = None
        i1 = code.find("//", pos)
        i2 = code.find("/*", pos)
        if i1 >= 0:
            lookfor = "\n"
            comment_start = i1
        if i2 >= 0:
            if not (i1 >= 0 and i1 < i2):
                lookfor = "*/"
                comment_start = i2
        # Found a start?
        if not lookfor:
            new_code += code[pos:]
            break
        else:
            new_code += code[pos:comment_start]
        # Find the end
        comment_end = code.find(lookfor, comment_start + 2)
        if comment_end < 0:
            break
        if lookfor == "\n":
            pos = comment_end
        else:
            pos = comment_end + len(lookfor)
    return new_code


# %% Functions to collect info from the headerfile


def collect_function_args():
    """ Get a dict that maps function-names to another dict that maps arg-names to c-arg strings.
    """

    funcs = {}

    for filename in [
        "ffi/webgpu-headers/webgpu.h",
        # "ffi//wgpu.h",
    ]:
        with open(filename, "rb") as f:
            text = f.read().decode()

        for line in text.splitlines():
            if line.startswith("WGPU_EXPORT"):
                funcname = line.split("(")[0].split(" ")[-1]
                funcs[funcname] = {}
                args = line.partition("(")[2].rpartition(")")[0]
                args = [arg.strip() for arg in args.split(",")]
                for arg in args:
                    if "*" in arg:
                        argname = arg.split()[-1].replace("_", "").lower()
                        funcs[funcname][argname] = arg

    return funcs


def collect_structs():
    """ Get a dict that maps struct names to another dict that maps field-names to c-type string.
    """

    structs = {}

    for filename in [
        "ffi/webgpu-headers/webgpu.h",
        # "ffi//wgpu.h",
    ]:
        with open(filename, "rb") as f:
            code = f.read().decode()

            i1 = i2 = i3 = i4 = 0
            while True:
                # Find struct
                i1 = code.find("typedef struct", i4)
                i2 = code.find("{", i1)
                i3 = code.find("}", i2)
                i4 = code.find(";", i3)
                if i1 < 0:
                    break
                # Only do simple structs, not Unions
                if 0 < code.find("{", i2 + 1) < i3:
                    continue
                # Decompose
                name = code[i3 + 1 : i4].strip()
                structs[name] = struct = {}
                for f in code[i2 + 1 : i3].strip().strip(";").split(";"):
                    parts = remove_c_comments(f).strip().split()
                    typename = " ".join(parts[:-1])
                    typename = typename.replace("const ", "")
                    key = parts[-1].strip("*")
                    struct[key] = typename

    return structs


# %% Checks


def check_rust_wrapper_args():
    """ Test that the wrappers use pointers where the c-header specifies pointers.
    """

    funcs = collect_function_args()

    fails = []

    for fname in os.listdir("src"):
        filename = os.path.join("src", fname)
        if not filename.endswith(".rs"):
            continue

        print(f"== Checking {fname}")
        with open(filename, "rb") as f:
            text = f.read().decode()

        for funcname in funcs:
            if not funcs[funcname]:
                continue  # no need to check
            i0 = text.find(funcname)
            if i0 > 0:
                print(f"In {funcname}:")
                i1 = text.find("(", i0)
                i2 = text.find(")", i1)
                args = " ".join(text[i1+1:i2].split())
                args = [arg.strip() for arg in args.split(",")]
                covered = set()
                for arg in args:
                    argname, _, typ = arg.partition(":")
                    argname = argname.replace("_", "").lower()
                    typ = typ.strip()
                    covered.add(argname)
                    if argname in funcs[funcname]:
                        # This arg is a pointer in C, so we must make it a ref or pointer
                        c_arg = funcs[funcname][argname]
                        ok = typ.startswith(("&", "*"))
                        message = f"  {'✔' if ok else '✖'} {argname}: {typ}"
                        print(message)
                        if not ok:
                            fails.append(message)
                        #print(f"  {status} {argname}: {typ}\n       {' '*len(argname)}{c_arg}")
                missing = set(funcs[funcname]).difference(covered)
                if missing:
                    message = f"{funcname} has missing args: {missing}"
                    print(message)
                    fails.append(message)

    return fails


def check_overloaded_structs():
    """ Check that the overloaded structs in build.rs have fields that match the headers.
    """

    structs = collect_structs()

    fails = []

    print(f"== Checking build.rs")

    with open("build.rs", "rb") as f:
        code = f.read().decode()

    name = None
    for line in code.splitlines():
        sline = line.strip()
        if sline.startswith("pub struct "):
            name = sline.split()[2]
            keys = set()
        elif name:
            if sline.startswith("}"):
                # Perform checks
                fail = None
                if name not in structs:
                    fail = f"✖ Unknown build.rs struct {name}"
                else:
                    ref_keys = set(structs[name])
                    mismatch = (ref_keys - keys) | (keys - ref_keys)
                    if mismatch:
                        fail = f"✖ Key mismatch in build.rs struct {name}: {mismatch}"
                # Report
                if fail:
                    fails.append(fail)
                    print(fail)
                else:
                    print(f"✔ Checked build.rs struct {name}")
                # Reset
                name = None
            elif sline.startswith("pub "):
                key = sline.replace(":", " ").split()[1]
                keys.add(key)

    return fails


# %% Script entrypoint


def main():

    fails = []
    fails += check_rust_wrapper_args()
    fails += check_overloaded_structs()

    print("=" * 20, "Summary", "=" * 20)

    if fails:
        print(f"There were {len(fails)} failed checks:")
        for fail in fails:
            print(fail)
        print("Failed")
        sys.exit(1)
    else:
        print(f"All checks passed.")
        sys.exit(0)


if __name__ == "__main__":
    main()
