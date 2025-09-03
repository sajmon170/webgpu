@group(0) @binding(0) var<uniform> uInput: BindingInput;
@group(0) @binding(1) var text: texture_2d<f32>;
@group(0) @binding(2) var sampl: sampler;

struct BindingInput {
    projection: mat4x4f,
    view: mat4x4f,
    model: mat4x4f,
    normal: mat4x4f,
    camera_pos: vec3f,
    time: f32
}

struct VertexInput {
    @location(0) pos: vec3f,
    @location(1) normal: vec3f,
    @location(2) uv: vec2f,
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) normal: vec3f,
    @location(1) uv: vec2f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let out = uInput.projection * uInput.view * uInput.model * vec4f(in.pos, 1.0);
    let normal = (uInput.normal * vec4f(in.normal, 0.0)).xyz;
    return VertexOutput(out, normal, in.uv);
}

@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) face: bool) -> @location(0) vec4f {
    let sample = textureSample(text, sampl, in.uv);
    let light = vec3f(1.0, 0.0, 0.0);
    let normal = normalize(in.normal);
    return dot(light, normal) * sample;
}
