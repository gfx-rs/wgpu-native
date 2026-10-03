"""Check public flag ABI types and native limit defaults in C and C++."""

import argparse
import os
from pathlib import Path
import re
import subprocess
import tempfile


ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--native-source", type=Path, default=ROOT)
    root_source = parser.parse_args().native_source.resolve()
    header = (root_source / "ffi/wgpu.h").read_text()
    limits = re.search(r"typedef struct WGPUNativeLimits\s*\{(.*?)\} WGPUNativeLimits;", header, re.S)
    if limits is None:
        raise RuntimeError("WGPUNativeLimits declaration was not found")
    fields = re.findall(r"uint32_t\s+(\w+)\s*;", limits.group(1))
    if not fields:
        raise RuntimeError("No native limit fields were found")
    checks = "\n".join(f"    assert(limits.{field} == WGPU_LIMIT_U32_UNDEFINED);" for field in fields)
    source = """#include <assert.h>
#include <stddef.h>
#include "wgpu.h"
#ifdef __cplusplus
#include <type_traits>
#define ABI_ASSERT static_assert
#define CHECK_FLAG_TYPE(T) static_assert(std::is_same<T, WGPUFlags>::value, #T " must alias WGPUFlags")
static_assert(std::is_same<decltype(&wgpuGetWgslLanguageFeatures), WGPUFlags (*)(void)>::value,
              "WGSL feature query must return WGPUFlags");
#else
#define ABI_ASSERT _Static_assert
#define CHECK_FLAG_TYPE(T) _Static_assert(_Generic((T)0, WGPUFlags: 1, default: 0), #T " must alias WGPUFlags")
_Static_assert(_Generic(&wgpuGetWgslLanguageFeatures, WGPUFlags (*)(void): 1, default: 0),
               "WGSL feature query must return WGPUFlags");
#endif
CHECK_FLAG_TYPE(WGPUNativeTextureFormatFeatureFlags);
CHECK_FLAG_TYPE(WGPUWgslLanguageFeatures);
CHECK_FLAG_TYPE(WGPUDownlevelFlags);
CHECK_FLAG_TYPE(WGPUWgpuBufferUsage);
CHECK_FLAG_TYPE(WGPUWgpuTextureUsage);
typedef struct {
    WGPUChainedStruct chain;
    WGPUFlags usage;
} ExpectedUsageExtras;
ABI_ASSERT(sizeof(WGPUWgpuBufferUsageExtras) == sizeof(ExpectedUsageExtras), "buffer usage extras size");
ABI_ASSERT(sizeof(WGPUWgpuTextureUsageExtras) == sizeof(ExpectedUsageExtras), "texture usage extras size");
ABI_ASSERT(offsetof(WGPUWgpuBufferUsageExtras, usage) == offsetof(ExpectedUsageExtras, usage), "buffer usage offset");
ABI_ASSERT(offsetof(WGPUWgpuTextureUsageExtras, usage) == offsetof(ExpectedUsageExtras, usage), "texture usage offset");
typedef struct {
    WGPUFlags flags;
    WGPUShaderModel shaderModel;
} ExpectedDownlevelCapabilities;
typedef struct {
    WGPUTextureUsage allowedUsages;
    WGPUFlags flags;
    WGPUFlags allowedWgpuUsages;
} ExpectedTextureFormatCapabilities;
ABI_ASSERT(sizeof(WGPUDownlevelCapabilities) == sizeof(ExpectedDownlevelCapabilities),
           "downlevel capabilities size");
ABI_ASSERT(offsetof(WGPUDownlevelCapabilities, shaderModel) == offsetof(ExpectedDownlevelCapabilities, shaderModel),
           "shader model follows a full-width flag field");
ABI_ASSERT(sizeof(WGPUNativeTextureFormatCapabilities) == sizeof(ExpectedTextureFormatCapabilities),
           "texture format capabilities size");
ABI_ASSERT(sizeof(((WGPUNativeTextureFormatCapabilities*)0)->flags) == sizeof(WGPUFlags),
           "texture format flags use the full field width, not tail padding");
int main(void) {
    WGPUNativeLimits limits = WGPU_NATIVE_LIMITS_INIT;
    assert(limits.chain.next == NULL);
    assert(limits.chain.sType == (WGPUSType)WGPUSType_NativeLimits);
""" + checks + """
    assert(WGPUWgpuBufferUsage_None == 0);
    assert(WGPUWgpuBufferUsage_BlasInput == 1);
    assert(WGPUWgpuBufferUsage_TlasInput == 2);
    assert(WGPUWgpuTextureUsage_None == 0);
    assert(WGPUWgpuTextureUsage_StorageAtomic == 1);
    assert(WGPUSType_WgpuBufferUsageExtras == 0x00030018);
    assert(WGPUSType_WgpuTextureUsageExtras == 0x00030019);
    return 0;
}
"""
    with tempfile.TemporaryDirectory(prefix="wgpu-native-headers-") as temporary:
        root = Path(temporary)
        for compiler, standard, suffix in [
            (os.environ.get("CC", "cc"), "c11", "c"),
            (os.environ.get("CXX", "c++"), "c++17", "cpp"),
        ]:
            translation_unit = root / f"limits.{suffix}"
            translation_unit.write_text(source)
            executable = root / f"limits-{suffix}"
            subprocess.run([
                compiler, f"-std={standard}", "-Wall", "-Wextra", "-Werror",
                f"-I{root_source / 'ffi'}", f"-I{root_source / 'ffi/webgpu-headers'}",
                str(translation_unit), "-o", str(executable),
            ], check=True)
            subprocess.run([str(executable)], check=True)
            print(f"{standard}: flag ABI checks and {len(fields)} native limit defaults passed")
        subprocess.run([
            os.environ.get("CXX", "c++"), "-std=c++17", "-Wall", "-Wextra", "-Werror", "-fsyntax-only",
            f"-I{root_source / 'ffi'}", f"-I{root_source / 'ffi/webgpu-headers'}",
            str(ROOT / "tests/native_usage.cpp"),
        ], check=True)
        print("c++17: direct native-usage consumer compiles")


if __name__ == "__main__":
    main()
