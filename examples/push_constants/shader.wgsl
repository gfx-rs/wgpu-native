@group(0)
@binding(0)
var<storage, read_write> buffer: array<u32>;

struct PushConstants {
    i: u32,
}
var<push_constant> push_constants: PushConstants;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = push_constants.i;
    buffer[i] = i * 2;
}
