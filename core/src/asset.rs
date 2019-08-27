use std::collections::HashMap;
use std::marker::PhantomData;
use std::process::Output;

use backend;
use interface::{
    Shader,
    VertexArray,
};
use crate::material::Material;

pub struct AssetPtr<T> {
    id: u32,
    pd: std::marker::PhantomData<fn() -> T>,
}

impl<T> Clone for AssetPtr<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            pd: PhantomData
        }
    }
}

impl<T> AssetPtr<T> {
    fn get_id(&self) -> u32 {
        self.id
    }
}

pub struct AssetHolder {
    vertex_array: AssetStorage<backend::VertexArray>,
    shader: AssetStorage<backend::Shader>,
    material: AssetStorage<Material>,
}

pub trait AssetMarker {
    fn select_storage(_: &AssetHolder) -> &AssetStorage<Self> where Self: Sized;
    fn select_storage_mut(_: &mut AssetHolder) -> &mut AssetStorage<Self> where Self: Sized;
}

impl AssetMarker for backend::VertexArray {
    fn select_storage(ah: &AssetHolder) -> &AssetStorage<Self> where Self: Sized {
        &ah.vertex_array
    }

    fn select_storage_mut(ah: &mut AssetHolder) -> &mut AssetStorage<Self> where Self: Sized {
        &mut ah.vertex_array
    }
}

impl AssetMarker for backend::Shader {
    fn select_storage(ah: &AssetHolder) -> &AssetStorage<Self> where Self: Sized {
        &ah.shader
    }

    fn select_storage_mut(ah: &mut AssetHolder) -> &mut AssetStorage<Self> where Self: Sized {
        &mut ah.shader
    }
}

impl AssetMarker for Material {
    fn select_storage(ah: &AssetHolder) -> &AssetStorage<Self> where Self: Sized {
        &ah.material
    }

    fn select_storage_mut(ah: &mut AssetHolder) -> &mut AssetStorage<Self> where Self: Sized {
        &mut ah.material
    }
}


impl AssetHolder {
    pub fn storage<T>(&self) -> &AssetStorage<T> where T: AssetMarker {
        T::select_storage(self)
    }
    pub fn storage_mut<T>(&mut self) -> &mut AssetStorage<T> where T: AssetMarker {
        T::select_storage_mut(self)
    }
}

impl Default for AssetHolder {
    fn default() -> Self {
        AssetHolder {
            vertex_array: Default::default(),
            shader: Default::default(),
            material: Default::default(),
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