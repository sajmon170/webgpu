@group(0) @binding(0) var<uniform> uInput: BindingInput;
@group(0) @binding(1) var text: texture_2d<f32>;
@group(0) @binding(2) var sampl: sampler;

struct BindingInput {
    xform: mat3x3f,
    time: f32
}

struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) uv: vec3f
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec3f
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let ratio = 640.0/480.0;
    let xform = uInput.xform;
    let rot = mat3x3f(cos(uInput.time), sin(uInput.time), 0, -sin(uInput.time), cos(uInput.time), 0, 0, 0, 1);
    let pos3d = vec3f(in.pos.x, in.pos.y, 1.0);
    let rotated = xform * rot * pos3d;
    let pos = vec4f((rotated.x - (uInput.time % 6.5) + 3.0) / ratio, rotated.y - (uInput.time % 6.5) + 3.0, in.pos.z, 1.0);
    return VertexOutput(pos, in.uv);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    var uv = in.uv.xy;
    let theta = 0.62;
    let rot = mat2x2f(cos(theta), sin(theta), -sin(theta), cos(theta));
    uv *= rot;
    uv *= 0.55;
    uv += vec2f(0.5, 0.5);
    return textureSample(text, sampl, uv);
}
