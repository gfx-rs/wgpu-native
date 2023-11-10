# %% Create instance

from wgpu_native import ffi, lib

instance_des = ffi.new("WGPUInstanceDescriptor *")
instance = lib.wgpuCreateInstance(instance_des)

# %% Create adapter

adapter = None


@ffi.callback("void(WGPURequestAdapterStatus, WGPUAdapter, char *, void *)")
def callback(status, result, message, userdata):
    if status != 0:
        msg = "-" if message == ffi.NULL else ffi.string(message).decode()
        assert False, f"Request adapter failed ({status}): {msg}"
    else:
        global adapter
        adapter = result


adapter_opts = ffi.new("WGPURequestAdapterOptions *")
adapter_opts.backendType = lib.WGPUBackendType_Undefined
adapter_opts.compatibleSurface = ffi.NULL
adapter_opts.forceFallbackAdapter = False
adapter_opts.powerPreference = lib.WGPUPowerPreference_HighPerformance
lib.wgpuInstanceRequestAdapter(instance, adapter_opts, callback, ffi.NULL)

assert adapter is not None

supported_limits = ffi.new("WGPUSupportedLimits *")
lib.wgpuAdapterGetLimits(adapter, supported_limits)

# %% Create device

device = None


@ffi.callback("void(WGPURequestDeviceStatus, WGPUDevice, char *, void *)")
def callback(status, result, message, userdata):
    if status != 0:
        msg = "-" if message == ffi.NULL else ffi.string(message).decode()
        assert False, f"Request device failed ({status}): {msg}"
    else:
        global device
        device = result


device_des = ffi.new("WGPUDeviceDescriptor *")
device_des.requiredLimits = ffi.new("WGPURequiredLimits *")
device_des.requiredLimits.limits = supported_limits.limits
device_des.defaultQueue = ffi.new("WGPUQueueDescriptor *")[0]
lib.wgpuAdapterRequestDevice(adapter, device_des, callback, ffi.NULL)

assert device is not None

queue = lib.wgpuDeviceGetQueue(device)
