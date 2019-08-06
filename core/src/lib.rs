///maths
pub extern crate nalgebra as na;
pub extern crate nalgebra_glm as glm;
///ecs
extern crate specs;
#[macro_use]
extern crate specs_derive;

pub mod backend;
pub mod render;

///internal
pub mod run;
pub mod utils;
pub mod ecs;
pub mod loader;
pub mod asset;

pub type Matrix4f = na::Matrix4<f32>;
//extern crate imgui;
//extern crate imgui_opengl_renderer;
