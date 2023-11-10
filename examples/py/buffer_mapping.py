# %% Imports

from wgpu_native import ffi, lib, ptr2memoryview
from create_device import device, queue

# %% Create buffer

buffer_des = ffi.new("WGPUBufferDescriptor *")
buffer_des.size = 64
buffer_des.mappedAtCreation = True
buffer_des.usage = lib.WGPUBufferUsage_MapRead
buffer = lib.wgpuDeviceCreateBuffer(device, buffer_des)

# %% Write data (buffer is already mapped at creation)

data_ptr = lib.wgpuBufferGetMappedRange(buffer, 0, 64)
data = ptr2memoryview(data_ptr, 64)
for i in range(len(data)):
    data[i] = 42

del data
lib.wgpuBufferUnmap(buffer)


# %% Read data

status = 999


@ffi.callback("void(WGPUBufferMapAsyncStatus, void*)")
def callback(status_, user_data_p):
    global status
    status = status_


lib.wgpuBufferMapAsync(buffer, lib.WGPUMapMode_Read, 0, 64, callback, ffi.NULL)
lib.wgpuDevicePoll(device, True, ffi.NULL)
assert status == 0

data_ptr = lib.wgpuBufferGetMappedRange(buffer, 0, 64)
data = ptr2memoryview(data_ptr, 64)

print("All zeros (data has not been flushed):")
print(data.tolist())

del data
lib.wgpuBufferUnmap(buffer)

# %% Submit an empty command buffer

encoder_des = ffi.new("WGPUCommandEncoderDescriptor *")
encoder = lib.wgpuDeviceCreateCommandEncoder(device, encoder_des)
command_buffer_des = ffi.new("WGPUCommandBufferDescriptor *")
command_buffer = lib.wgpuCommandEncoderFinish(encoder, command_buffer_des)
lib.wgpuQueueSubmit(queue, 1, ffi.new("WGPUCommandBuffer []", [command_buffer]))

# %% Read data (again)

status = 999


@ffi.callback("void(WGPUBufferMapAsyncStatus, void*)")
def callback(status_, user_data_p):
    global status
    status = status_


lib.wgpuBufferMapAsync(buffer, lib.WGPUMapMode_Read, 0, 64, callback, ffi.NULL)
lib.wgpuDevicePoll(device, True, ffi.NULL)
assert status == 0

data_ptr = lib.wgpuBufferGetMappedRange(buffer, 0, 64)
data = ptr2memoryview(data_ptr, 64)

print("All 42's:")
print(data.tolist())

del data
lib.wgpuBufferUnmap(buffer)

# %%
