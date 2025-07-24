@vertex
fn vs_main(@location(0) vertex: vec3f) -> @builtin(position) vec4f {
    return vec4f(vertex, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4f {
    return vec4f(0.1, 0.1, 0.5, 1.0);
}
