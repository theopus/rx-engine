extern crate rx_engine;

use specs::{Builder, Join, Read, ReadStorage, WorldExt, Write, WriteStorage};

use rx_engine::{Matrix4f, specs};
use rx_engine::asset::AssetPtr;
use rx_engine::backend;
use rx_engine::ecs::{ActiveCamera, DeltaTime, PlatformEvents};
use rx_engine::ecs::components::{Camera, Position, Render, Rotation, Transformation};
use rx_engine::ecs::layer::EcsLayerBuilder;
use rx_engine::glm;
use rx_engine::interface::{BufferLayout, Event, shared_types, VertexArray, VertexBuffer, WindowConfig};
use rx_engine::interface::{RendererApi, RendererConstructor};
use rx_engine::loader::Loader;
use rx_engine::utils::relative_to_current_path;

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


fn main() {
    let mut engine = rx_engine::run::build_engine(
        WindowConfig { width: 600, height: 400 },
        EcsLayerBuilder::new(Box::new(|mut w, d, ctx| {
            let mut path_buf = &relative_to_current_path(&vec!["client", "resources", "cube.obj"]);
            let mut loader = Loader;
            let result = loader.load_obj(path_buf);

            let mut vertex_array: backend::VertexArray = ctx.renderer_constructor.vertex_array();
            let mut ib = ctx.renderer_constructor.index_buffer(&result.indices);
            let mut vb: Box<backend::VertexBuffer> = Box::new(ctx.renderer_constructor.vertex_buffer());

            vertex_array.set_index_buffer(ib);
            vb.buffer_data_f32(&result.positions);
            vb.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
            vertex_array.add_vertex_buffer(*vb);

            ctx.renderer.api().set_clear_color(0.3, 0.3, 0.9, 1_f32);

            let va_ptr: AssetPtr<backend::VertexArray> = ctx.asset_holder.storage_mut().put(vertex_array);


            let shader: backend::Shader = ctx.renderer_constructor.shader(
                &relative_to_current_path(&vec!["client", "src", "test", "vert.glsl"]),
                &relative_to_current_path(&vec!["client", "src", "test", "frag.glsl"]),
                &BufferLayout::with(shared_types::FLOAT_3));

            let shader: AssetPtr<backend::Shader> = ctx.asset_holder.storage_mut().put(shader);


            let d = d.with(CameraSystem, "camera_system", &[]);

            w.create_entity()
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

            w.create_entity()
                .with(Transformation {
                    mtx: glm::identity()
                })
                .with(Position {
                    x: 5.0,
                    y: 0.0,
                    z: 0.0,
                })
                .with(Rotation {
                    x: (0.0 + (0 as f32)),
                    y: 0.0,
                    z: 0.0,
                })
                .with(Render {
                    va: va_ptr.clone(),
                    shader: shader.clone(),
                })
                .build();


            return (w, d);
        })),
    );
    dbg!();


    engine.run();
    println!("Bye!")
}
