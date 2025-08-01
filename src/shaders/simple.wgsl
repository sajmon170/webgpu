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
    let xformed = uInput.xform * in.pos;
    let pos = vec4f(xformed. / ratio, xformed.y, in.pos.z, 1.0);
    return VertexOutput(pos, in.uv);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    var uv = in.uv.xy;
    let sample = textureSample(text, sampl, uv);
  
    if sample.a == 0 {
       discard;
    }
  
    return sample;
}
