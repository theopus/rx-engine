extern crate rx_engine;

use specs::{Builder, Join, Read, ReadStorage, WorldExt, WriteStorage, Write};

use rx_engine::{Matrix4f, specs};
use rx_engine::ecs::components::{Camera, Position, Rotation, Transformation};
use rx_engine::ecs::layer::EcsLayerBuilder;
use rx_engine::ecs::{PlatformEvents, ActiveCamera, DeltaTime};
use rx_engine::glm;
use rx_engine::interface::{Event, WindowConfig};

mod sandbox_layer;


pub struct EmptySystem;

impl<'a> rx_engine::specs::System<'a> for EmptySystem {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Rotation>,
                       WriteStorage<'a, Transformation>);
    fn run(&mut self, data: Self::SystemData) {
        println!("Im retard");
    }
}

pub struct CameraSystem;

impl<'a> rx_engine::specs::System<'a> for CameraSystem {
    type SystemData = (ReadStorage<'a, Position>,
                       WriteStorage<'a, Rotation>,
                       WriteStorage<'a, Camera>,
                       Read<'a, PlatformEvents>,
                       Write<'a, ActiveCamera>,
                       Read<'a, DeltaTime>,
    );

    fn run(&mut self, (pos, mut rot, mut camera, events, mut active, delta): Self::SystemData) {
        for (pos, rot, camera) in (&pos, &mut rot, &mut camera).join() {
            let projection: Option<Matrix4f> = None;

            for e in events.0.iter() {
                if let Event::Resize(w, h) = e {
                    camera.projection = {
                        glm::perspective(
                            (*w) as f32 / (*h) as f32,
                            glm::radians(&glm::vec1(camera.fov)).x,
                            0.1,
                            1000.,
                        )
                    };
                }
            }

            camera.view = {
                let mut mtx: Matrix4f = glm::identity();

                mtx = glm::translate(&mtx, &glm::vec3(pos.x, pos.y, pos.z));
                mtx = glm::rotate(&mtx, glm::radians(&glm::vec1(rot.x)).x, &glm::vec3(1., 0., 0.));
                mtx = glm::rotate(&mtx, glm::radians(&glm::vec1(rot.y)).x, &glm::vec3(0., 1., 0.));
                mtx = glm::rotate(&mtx, glm::radians(&glm::vec1(rot.z)).x, &glm::vec3(0., 0., 1.));
                glm::inverse(&mtx)
//                mtx
            };
            active.view_mtx = camera.view;
            active.proj_mtx = camera.projection;
        }
    }
}


fn main() {
    let mut engine = rx_engine::run::build_engine(
        WindowConfig { width: 600, height: 400 },
        EcsLayerBuilder::new(Box::new(|mut w, d| {
            let d = d.with(CameraSystem, "camera_system", &[]);
            let ball = w.create_entity()
                .with(Camera::default())
                .with(Position {
                    x: 0.0,
                    y: 0.0,
                    z: 10.0,
                })
                .with(Rotation {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                })
                .build();
            return (w, d);
        })),
    );
    engine.add_layer_builder(sandbox_layer::SandboxLayerBuilder);
    engine.run();
    println!("Bye!")
}
