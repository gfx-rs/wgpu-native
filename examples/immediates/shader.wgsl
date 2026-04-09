@group(0)
@binding(0)
var<storage, read_write> buffer: array<u32>;

struct Immediates {
    i: u32,
}
var<immediate> immediates: Immediates;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = immediates.i;
    buffer[i] = i * 2;
}
