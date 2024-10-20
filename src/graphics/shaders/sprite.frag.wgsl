// sprite.frag.wgsl

@group(0) @binding(0) var my_texture: texture_2d<f32>;
@group(0) @binding(1) var my_sampler: sampler;

struct FragmentInput {
    @location(0) tex_coords: vec2<f32>,
};

@fragment
fn main(input: FragmentInput) -> @location(0) vec4<f32> {
    return textureSample(my_texture, my_sampler, input.tex_coords); // Aplica a textura usando UVs
}
