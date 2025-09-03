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
    @location(1) view_direction: vec3f,
    @location(2) uv: vec2f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let world_pos = uInput.model * vec4f(in.pos, 1.0);
    let out_pos = uInput.projection * uInput.view * world_pos;
    let normal = (uInput.normal * vec4f(in.normal, 0.0)).xyz;
    let view_direction = uInput.camera_pos - world_pos.xyz;
    return VertexOutput(out_pos, normal, view_direction, in.uv);
}

@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) face: bool) -> @location(0) vec4f {
    let light = vec3f(1.0, 0.0, 0.0); 
    let sample = textureSample(text, sampl, in.uv);
    let normal = normalize(in.normal);
    
    let diffuse = max(0.0, dot(light, normal)) * sample;
    
    let R = reflect(-light, normal);
    let angle = max(0.0, dot(R, in.view_direction));
    let hardness = 10.0;
    let specular = vec4f(vec3f(pow(angle, hardness)), 1.0);
    
    return diffuse + specular;
}
