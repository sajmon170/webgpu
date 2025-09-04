@group(0) @binding(0) var<uniform> uInput: BindingInput;
@group(0) @binding(1) var text: texture_2d<f32>;
@group(0) @binding(2) var norm: texture_2d<f32>;
@group(0) @binding(3) var sampl: sampler;

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
    @location(1) tangent: vec3f,
    @location(2) bitangent: vec3f,
    @location(3) normal: vec3f,
    @location(4) uv: vec2f,
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) tangent: vec3f,
    @location(1) bitangent: vec3f,
    @location(2) normal: vec3f,
    @location(3) view_direction: vec3f,
    @location(4) uv: vec2f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let world_pos = uInput.model * vec4f(in.pos, 1.0);
    let out_pos = uInput.projection * uInput.view * world_pos;
    let normal = (uInput.normal * vec4f(in.normal, 0.0)).xyz;
    let view_direction = normalize(uInput.camera_pos - world_pos.xyz);

    let tangent = (uInput.normal * vec4f(in.tangent, 0.0)).xyz;
    let bitangent = (uInput.normal * vec4f(in.bitangent, 0.0)).xyz;

    return VertexOutput(out_pos, tangent, bitangent, normal, view_direction, in.uv);
}

@fragment
fn fs_main(in: VertexOutput, @builtin(front_facing) face: bool) -> @location(0) vec4f {
    let light = vec3f(10.0, 0.0, 0.0); 
    let texture_sample = textureSample(text, sampl, in.uv);
    let normal_sample = textureSample(norm, sampl, in.uv);
    let local_normal = normal_sample.rgb * 2.0 - 1.0;
    let local_to_world = mat3x3f(
        normalize(in.tangent),
        normalize(in.bitangent),
        normalize(in.normal)
    );
    let world_normal = local_to_world * local_normal;
    let strength = 0.5;
    let normal = mix(in.normal, world_normal, strength);
    
    let diffuse = max(0.3, dot(-light, normal)) * texture_sample;
    //let diffuse = vec4f(world_normal, 1.0);
    
    let R = normalize(reflect(-light, normal));
    let angle = max(0.0, dot(R, in.view_direction));
    let hardness = 32.0;
    let specular = 0.6*vec4f(vec3f(pow(angle, hardness)), 1.0);
    
    return diffuse + specular;
}
