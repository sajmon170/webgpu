@group(0) @binding(0) var<uniform> uInput: BindingInput;
@group(0) @binding(1) var text: texture_2d<f32>;
@group(0) @binding(2) var sampl: sampler;

struct BindingInput {
    xform: mat4x4f,
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

    let focalLength = 2.0;
    let near = 0.01;
    let far = 100.0;
    let P = transpose(mat4x4f(
        focalLength,         0.0,                0.0,                   0.0,
            0.0,     focalLength * ratio,        0.0,                   0.0,
            0.0,             0.0,         far / (far - near), -far * near / (far - near),
            0.0,             0.0,                1.0,                   0.0,
    ));


    let out = P * uInput.xform * vec4f(in.pos, 1.0);
  
    return VertexOutput(out, in.uv);
}

@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) face: bool) -> @location(0) vec4f {
    if (face) {
      let col = 1.0 - in.pos.z;
      return vec4f(col, col, col, 1.0);
    }
    else {
      return vec4f(1.0, 0.0, 0.0, 1.0);
    }
}
