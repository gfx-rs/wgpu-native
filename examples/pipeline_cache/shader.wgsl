@group(0)
@binding(0)
var<storage, read_write> values: array<u32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    values[global_id.x] = values[global_id.x] * 2u;
}
