///internal
pub mod platform;
pub mod render;
pub mod open_gl;
pub mod run;
pub mod utils;
pub mod ecs;
pub mod loader;
///maths
pub extern crate nalgebra as na;
pub extern crate nalgebra_glm as glm;
pub type Matrix4f = na::Matrix4<f32>;
///graphics api and platforms
extern crate gl;
extern crate glfw;
///ecs
extern crate specs;
#[macro_use]
extern crate specs_derive;
