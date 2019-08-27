use crate::{backend, Vec3f, Matrix4f};
use crate::interface::Shader;
use std::collections::HashMap;
use crate::asset::{AssetPtr, AssetHolder, AssetStorage};

pub struct Material {
    shader: backend::Shader
}

type Upload = Box<Fn()>;
type UploadRaw<'a> = dyn Fn(&'a backend::Shader) -> ();

fn to_upload<'a, F>(f: F) -> Upload
    where F: Fn(&'a backend::Shader) -> () + 'static {
    Box::new(f) as Upload
}

pub struct MaterialInstance {
    material: AssetPtr<Material>,
    uploads: HashMap<&'static str, Upload>,
}

impl MaterialInstance {
    pub fn new(material: AssetPtr<Material>) -> Self {
        MaterialInstance { material, uploads: HashMap::new() }
    }

    pub fn material(&self) -> &AssetPtr<Material> {
        &self.material
    }
}

impl MaterialInstance {
        pub fn set_mat4(&mut self, placeholder: &'static str, value: &Matrix4f) {
            let mtx: Matrix4f = glm::identity() * value;
            self.uploads.insert(placeholder,
                                to_upload(move |shader: &backend::Shader| {
                                    shader.load_mat4(placeholder, mtx.as_slice());
                                }));
    }

    pub fn prepare(&self, material: & Material) {
        let shader = material.shader();
        for upload in self.uploads.values() {
            upload(shader);
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

    fn shader(&self) -> &backend::Shader {
        &self.shader
    }
}