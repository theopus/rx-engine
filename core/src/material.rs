use std::collections::HashMap;

use crate::{backend, Matrix4f, Vec3f};
use crate::asset::{AssetHolder, AssetPtr, AssetStorage};
use crate::interface::Shader;

pub struct Material {
    shader: backend::Shader
}

#[derive(Clone)]
pub struct MaterialInstance {
    material: AssetPtr<Material>,
    uploads_mat4: HashMap<&'static str, Matrix4f>,
    uploads_vec3: HashMap<&'static str, Vec3f>,
}


impl MaterialInstance {
    pub fn new(material: AssetPtr<Material>) -> Self {
        MaterialInstance {
            material,
            uploads_mat4: HashMap::new(),
            uploads_vec3: HashMap::new(),
        }
    }

    pub fn material(&self) -> &AssetPtr<Material> {
        &self.material
    }
}

impl MaterialInstance {
    pub fn set_mat4(&mut self, placeholder: &'static str, value: &Matrix4f) {
        let mtx: Matrix4f = glm::identity() * value;
        self.uploads_mat4.insert(placeholder, mtx);
    }
    pub fn set_vec3(&mut self, placeholder: &'static str, value: &Vec3f) {
        self.uploads_vec3.insert(placeholder, value.clone());
    }

    pub fn prepare(&self, material: &Material) {
        let shader: &backend::Shader = material.shader();
        for (p, upload) in self.uploads_mat4.iter() {
            shader.load_mat4(p, upload.as_slice())
        }
        for(p, upload) in self.uploads_vec3.iter(){
            shader.load_vec3(p, upload.as_slice())
        }
    }
}

impl AssetPtr<Material> {
    pub fn instance(&self) -> MaterialInstance {
        MaterialInstance::new(self.clone())
    }
}

impl Material {
    pub fn from_shader(shader: backend::Shader) -> Self {
        Material { shader }
    }

    pub fn bind(&self) {
        self.shader.bind();
    }

    pub fn shader(&self) -> &backend::Shader {
        &self.shader
    }
}