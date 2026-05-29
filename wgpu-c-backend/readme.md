# wgpu-c-backend

A custom wgpu backend routing through the wgpu-native C API, used to verify API completeness.

Features which are unimplemented in wgpu-native will have a comment explaining that they cannot be implemented in wgpu-c-backend.

## Directory structure

The inner `wgpu-c-backend/` folder exists so that the crate can depend on `wgpu-native` two directories up. When cloning into wgpu's tree, it will connect to both the local wgpu and the local wgpu-native correctly.
