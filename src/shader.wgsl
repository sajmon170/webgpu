@group(0) @binding(0) var<uniform> utime: f32;

struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) color: vec3f
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) color: vec3f
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let ratio = 640.0/480.0;
    let scale = mat2x2f(0.75, 0, 0, 0.75);
    let rot = mat2x2f(cos(utime), sin(utime), -sin(utime), cos(utime));
    let pos2d = vec2f(in.pos.x, in.pos.y);
    let rotated = scale * rot * pos2d;
    let pos = vec4f(rotated.x / ratio, rotated.y, in.pos.z, 1.0);
    return VertexOutput(pos, in.color);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let srgb = pow(in.color, vec3f(2.2));
    return vec4f(srgb, 1.0);
}
