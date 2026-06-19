@group(0) @binding(0) var<storage, read> input: array<vec3<f32>>;
@group(0) @binding(1) var<storage, read_write> output: array<vec3<f32>>;

@compute
@workgroup_size(8, 8, 1)
fn main(
    @builtin(local_invocation_id) local_id: vec3<u32>,
    @builtin(workgroup_id) group_id: vec3<u32>
) {
    let local_index = local_id.y * 8u + local_id.x;
    let global_index = group_id.x * 64u + local_index;

    if (global_index >= arrayLength(&input)) {
        return;
    }

    let x = global_index % 256u;
    let y = global_index / 256u;

    output[global_index].x = f32(x) / 256.0;
    output[global_index].y = f32(y) / 256.0;
}