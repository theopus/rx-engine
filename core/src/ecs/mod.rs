use specs::Entities;
use specs::Read;
use specs::System;

use crate::ecs::components::Camera;
use crate::interface::Event;
use crate::Matrix4f;

pub mod layer;

//resources
#[derive(Default)]
pub struct DeltaTime(pub f64);

#[derive(Default)]
pub struct PlatformEvents(pub Vec<Event>);

pub struct ActiveCamera {
    pub view_mtx: Matrix4f,
    pub proj_mtx: Matrix4f,
}

impl Default for ActiveCamera {
    fn default() -> Self {
        Self {
            view_mtx: glm::identity(),
            proj_mtx: glm::identity(),
        }
    }
}

//components
pub mod components {
    use na::Matrix4;
    use specs::{Component, VecStorage};

    use crate::ecs::ActiveCamera;

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

    #[derive(Component, Debug)]
    #[storage(VecStorage)]
    pub struct Camera {
        pub fov: f32,

        pub view: Matrix4<f32>,
        pub projection: Matrix4<f32>,
    }

    impl Camera {
        fn default_with_aspect(aspect_ratio: f32) -> Self {
            Self {
                fov: 45.,
                view: glm::identity(),
                projection: glm::perspective(
                    aspect_ratio, glm::radians(&glm::vec1(45.)).x,
                    0.1, 1000., ),
            }
        }
    }

    impl Default for Camera {
        fn default() -> Self {
            Camera::default_with_aspect(6. / 4.)
        }
    }
}
