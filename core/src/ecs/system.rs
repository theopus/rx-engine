use specs::{
    Dispatcher,
    DispatcherBuilder,
    Join,
    Read,
    ReadStorage,
    System,
    World,
    WorldExt,
    Write,
    WriteStorage,
};

use crate::{
    ecs::{
        ActiveCamera,
        components::{Camera, Position, Rotation},
        DeltaTime,
        PlatformEvents,
    },
    api::Event,
    Matrix4f,
};
use crate::ecs::components::{Transformation, Velocity};

pub struct CameraSystem;

impl<'a> specs::System<'a> for CameraSystem {
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
                mtx = glm::translate(&mtx, &glm::vec3(pos.x, pos.y, pos.z)); // camera translate
                mtx = glm::rotate(&mtx, glm::radians(&glm::vec1(rot.x)).x, &glm::vec3(1., 0., 0.)); //camera rot
                mtx = glm::rotate(&mtx, glm::radians(&glm::vec1(rot.y)).x, &glm::vec3(0., 1., 0.));
                mtx = glm::rotate(&mtx, glm::radians(&glm::vec1(rot.z)).x, &glm::vec3(0., 0., 1.));
                glm::inverse(&mtx)
            };
            active.view_mtx = camera.view;
            active.proj_mtx = camera.projection;
        }
    }
}

pub struct TransformationSystem;


impl<'a> System<'a> for TransformationSystem {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Rotation>,
                       WriteStorage<'a, Transformation>);

    fn run(&mut self, (pos, rot, mut tsm): Self::SystemData) {
        for (pos, rot, tsm) in (&pos, &rot, &mut tsm).join() {
            tsm.mtx = {
                let mut mtx = glm::identity();
                glm::rotate(&mut mtx, rot.x, &glm::vec3(1., 0., 0.)) *
                    glm::rotate(&mut mtx, rot.y, &glm::vec3(0., 1., 0.)) *
                    glm::rotate(&mut mtx, rot.z, &glm::vec3(0., 0., 1.)) *
                    glm::translate(&mut mtx, &glm::vec3(pos.x, pos.y, pos.z))
            };
        }
    }
}

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (WriteStorage<'a, Position>,
                       WriteStorage<'a, Velocity>,
                       Read<'a, DeltaTime>);

    fn run(&mut self, (mut pos, mut vel, delta): Self::SystemData) {
        let slow_rate = 1.0;

        for (pos, vel) in (&mut pos, &mut vel).join() {
            pos.x += (vel.x as f64 * delta.0) as f32;
            pos.y += (vel.y as f64 * delta.0) as f32;
            pos.z += (vel.z as f64 * delta.0) as f32;
        }
    }
}

