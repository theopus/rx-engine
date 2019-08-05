use specs::Entities;
use specs::Read;
use specs::System;

pub mod layer;

//resources
#[derive(Default)]
pub struct DeltaTime(pub f64);

//components
pub mod components {
    use na::Matrix4;
    use specs::{Component, VecStorage};

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Position {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Rotation {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct RotationVelocity {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Velocity {
        pub x: f32,
        pub y: f32,
        pub z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Transformation {
        pub mtx: Matrix4<f32>
    }
}