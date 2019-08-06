use std::collections::HashMap;
use std::marker::PhantomData;
use std::process::Output;

use crate::backend::{Shader, VertexArray};

pub struct AssetPtr<T> {
    id: u32,
    pd: std::marker::PhantomData<T>,
}

impl<T> AssetPtr<T> {
    fn get_id(&self) -> u32 {
        self.id
    }
}

pub struct AssetHolder {
    vertex_array: AssetStorage<VertexArray>,
    shader: AssetStorage<Shader>,
}


impl AssetHolder {
    pub fn vertex_array_mut(&mut self) -> &mut AssetStorage<VertexArray> {
        &mut self.vertex_array
    }
    pub fn shader_mut(&mut self) -> &mut AssetStorage<Shader> {
        &mut self.shader
    }
    pub fn vertex_array(&self) -> &AssetStorage<VertexArray> {
        &self.vertex_array
    }
    pub fn shader(&self) -> &AssetStorage<Shader> {
        &self.shader
    }
}

impl Default for AssetHolder {
    fn default() -> Self {
        AssetHolder {
            vertex_array: Default::default(),
            shader: Default::default(),
        }
    }
}

pub struct AssetStorage<T> {
    storage_map: HashMap<u32, T>
}

impl<T> Default for AssetStorage<T> {
    fn default() -> Self {
        AssetStorage {
            storage_map: HashMap::new()
        }
    }
}

impl<T> AssetStorage<T> {
    pub fn put(&mut self, asset: T) -> AssetPtr<T> {
        let ptr = AssetPtr {
            id: self.storage_map.len() as u32,
            pd: PhantomData,
        };
        self.storage_map.insert(ptr.id, asset);
        ptr
    }

    pub fn get_ref(&self, asset_ptr: &AssetPtr<T>) -> Option<&T> {
        self.storage_map.get(&asset_ptr.id)
    }

    pub fn get_ref_mut(&mut self, asset_ptr: &AssetPtr<T>) -> Option<&mut T> {
        self.storage_map.get_mut(&asset_ptr.id)
    }

    pub fn remove(&mut self, asset_ptr: &AssetPtr<T>) -> Option<T> {
        self.storage_map.remove(&asset_ptr.id)
    }
}