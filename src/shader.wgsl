@group(0) @binding(0) var<uniform> uInput: BindingInput;

struct BindingInput {
    xform: mat3x3f,
    time: f32
}

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
    let xform = uInput.xform;
    let rot = mat3x3f(cos(uInput.time), sin(uInput.time), 0, -sin(uInput.time), cos(uInput.time), 0, 0, 0, 1);
    let pos3d = vec3f(in.pos.x, in.pos.y, 1.0);
    let rotated = xform * rot * pos3d;
    let pos = vec4f((rotated.x - (uInput.time % 6.5) + 3.0) / ratio, rotated.y - (uInput.time % 6.5) + 3.0, in.pos.z, 1.0);
    return VertexOutput(pos, in.color);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    let srgb = pow(in.color, vec3f(2.2));
    return vec4f(srgb, 1.0);
}
