extern crate gl_generator;

use std::env;
use std::fs::File;
use std::path::Path;
use gl_generator::{Api, DebugStructGenerator, Fallbacks, GlobalGenerator, Profile, Registry, StructGenerator};

fn main() {
    if false {
        let dest = env::var("OUT_DIR").unwrap();
        let mut file = File::create(&Path::new(&dest)
            .join("..")
            .join("..")
            .join("..")
            .join("..")
            .join("..")
            .join("..")
            .join("gl")
            .join("src")
            .join("lib.rs")
        )
            .unwrap();

        Registry::new(Api::Gl, (3, 3), Profile::Core, Fallbacks::All, [])
            .write_bindings(StructGenerator, &mut file)
            .unwrap();
    }
}