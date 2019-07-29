use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

use tobj::{Mesh, MTLLoadResult};

pub struct LoadResult {
    pub positions: Vec<f32>,
    pub uvs: Vec<f32>,
    pub normals: Vec<f32>,
    pub indices: Vec<u32>,
}

pub struct Loader;


impl Loader {
    pub fn load_obj(&mut self, path: &Path) -> LoadResult {
        println!("File: {}", path.to_str().unwrap());
        let mut a = BufReader::new(File::open(path.to_path_buf()).expect(""));
        let (models, materials) = tobj::load_obj_buf(&mut a, |p| -> MTLLoadResult {
            Result::Ok((Vec::new(), HashMap::new()))
        }).expect("");
        println!("# of models: {}", models.len());
        println!("# of materials: {}", materials.len());
        if models.len() > 1 {}

        let mesh: &Mesh = &models[0].mesh;

        println!("# of vertexes {}", mesh.positions.len() / 3);
        println!("# of indexes {}", mesh.indices.len());
        LoadResult {
            positions: mesh.positions.clone(),
            uvs: mesh.texcoords.clone(),
            normals: mesh.normals.clone(),
            indices: mesh.indices.clone(),
        }
    }
}