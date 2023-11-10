"""
Little library to instaniate the dynamic library.
The paths to the header files and DLL are taken relative to this file.
"""

import os
import sys
import ctypes

import cffi


this_dir = os.path.abspath(os.path.dirname(__file__))
repo_dir = os.path.dirname(os.path.dirname(this_dir))


def get_wgpu_lib_path():
    # If path is given, use that or fail trying
    override_path = os.getenv("WGPU_LIB_PATH", "").strip()
    if override_path:
        return override_path

    # Load the debug binary if requested
    build = "debug"

    # Get lib filename for supported platforms
    if sys.platform.startswith("win"):
        lib_filename = "wgpu_native.dll"
    elif sys.platform.startswith("darwin"):
        lib_filename = "libwgpu_native.dylib"
    elif sys.platform.startswith("linux"):
        lib_filename = "libwgpu_native.so"
    else:
        raise RuntimeError("No WGPU library found. Build it, or set WGPU_LIB_PATH.")

    return os.path.join(repo_dir, "target", build, lib_filename)


def get_wgpu_header(*filenames):
    """Combine headers into a single textual representation."""
    # Read files
    lines1 = []
    for filename in filenames:
        with open(filename) as f:
            lines1.extend(f.readlines())
    # Deal with pre-processor commands, because cffi cannot handle them.
    # Just removing them, plus a few extra lines, seems to do the trick.
    lines2 = []
    for line in lines1:
        if line.startswith("#define ") and len(line.split()) > 2 and "0x" in line:
            line = line.replace("(", "").replace(")", "")
        elif line.startswith("#"):
            continue
        elif 'extern "C"' in line:
            continue
        for define_to_drop in [
            "WGPU_EXPORT ",
            "WGPU_NULLABLE ",
            " WGPU_OBJECT_ATTRIBUTE",
            " WGPU_ENUM_ATTRIBUTE",
            " WGPU_FUNCTION_ATTRIBUTE",
            " WGPU_STRUCTURE_ATTRIBUTE",
        ]:
            line = line.replace(define_to_drop, "")
        lines2.append(line)
    return "\n".join(lines2)


WEBGPU_H_PATH = os.path.join(repo_dir, "ffi", "webgpu-headers", "webgpu.h")
WGPU_H_PATH = os.path.join(repo_dir, "ffi", "wgpu.h")
LIB_PATH = get_wgpu_lib_path()

header = get_wgpu_header(WEBGPU_H_PATH, WGPU_H_PATH)

ffi = cffi.FFI()
ffi.cdef(header)
ffi.set_source("wgpu.h", None)
lib = ffi.dlopen(LIB_PATH)


# %%% Some helper functions


def ptr2memoryview(ptr, nbytes, format="B"):
    """Get a memoryview from an ffi pointer and a byte count."""
    address = int(ffi.cast("intptr_t", ptr))
    c_array = (ctypes.c_uint8 * nbytes).from_address(address)
    return memoryview(c_array).cast(format, shape=(nbytes,))
