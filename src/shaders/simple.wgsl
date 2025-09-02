@group(0) @binding(0) var<uniform> uInput: BindingInput;
@group(0) @binding(1) var text: texture_2d<f32>;
@group(0) @binding(2) var sampl: sampler;

struct BindingInput {
    projection: mat4x4f,
    view: mat4x4f,
    model: mat4x4f,
    time: f32
}

struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) uv: vec2f
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) uv: vec2f
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let out = uInput.projection * uInput.view * uInput.model * vec4f(in.pos, 1.0);
    return VertexOutput(out, in.uv);
}

@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) face: bool) -> @location(0) vec4f {
    let sample = textureSample(text, sampl, in.uv);
    return sample;
}
