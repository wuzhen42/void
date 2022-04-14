use super::texture::Texture;
use wgpu;

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Mesh {
    pub name: String,
    pub vertex: wgpu::Buffer,
    pub index: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

pub struct Material {
    pub name: String,
    pub diffuse: Texture,
    pub bindgroup: wgpu::BindGroup,
}
