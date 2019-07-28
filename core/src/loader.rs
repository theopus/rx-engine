use std::path::Path;
use std::io::BufRead;

use std::fs::File;
use std::io::BufReader;
use tobj::{Mesh, MTLLoadResult};
use std::collections::HashMap;

pub struct LoadResult {
    positions: Vec<f32>,
    uvs: Vec<f32>,
    normals: Vec<f32>,
    indices: Vec<u32>,
}

pub struct Loader;


impl Loader {
    pub fn load_obj(&mut self, path: &mut Path) -> LoadResult {
        let a = BufReader::new(File::open(path.to_path_buf()).expect(""), |p| -> MTLLoadResult {
            Result::Ok((Vec::new(), HashMap::new()))
        });
        let (models, materials) =
            tobj::load_obj_buf(a;
        println!("File: {}", path.to_str().unwrap());
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