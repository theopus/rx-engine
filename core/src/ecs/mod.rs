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
    struct Position {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    struct Rotation {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    struct RotationVelocity {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    struct Velocity {
        x: f32,
        y: f32,
        z: f32,
    }

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    struct Transformation {
        mtx: Matrix4<f32>
    }
}