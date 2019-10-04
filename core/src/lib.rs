pub extern crate backend_api as api;
pub extern crate backend_opengl as backend;
pub extern crate imgui;
///maths
pub extern crate nalgebra as na;
pub extern crate nalgebra_glm as glm;
///ecs
pub extern crate specs;
pub extern crate rand;
#[macro_use]
pub extern crate specs_derive;

//pub use backend::*;

//pub mod backend;
pub mod render;

///internal
pub mod run;
pub mod utils;
pub mod ecs;
pub mod loader;
pub mod mesh;

mod layer;

pub type Matrix4f = na::Matrix4<f32>;
pub type Vec3f = na::Vector3<f32>;

