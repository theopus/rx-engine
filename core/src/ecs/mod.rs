use specs::System;
use specs::Entities;


struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = (
        Entities<'a>
    );

    fn run(&mut self, data: Self::SystemData) {
//        data.create()
    }
}

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