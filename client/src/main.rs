extern crate rx_engine;

use specs::{Builder, Join, Read, ReadStorage, WorldExt, Write, WriteStorage};

use rx_engine::{Matrix4f, specs};
use rx_engine::asset::AssetPtr;
use rx_engine::backend;
use rx_engine::ecs::{ActiveCamera, DeltaTime, InputEvent, InputEventsRead, InputEventsWrite, InputType, PlatformEvents};
use rx_engine::ecs::components::{Camera, Position, Render, Rotation, Transformation, Velocity};
use rx_engine::ecs::layer::EcsLayerBuilder;
use rx_engine::glm;
use rx_engine::interface::{Action, BufferLayout, Event, shared_types, VertexArray, VertexBuffer, WindowConfig};
use rx_engine::interface::{RendererApi, RendererDevice};
use rx_engine::loader::Loader;
use rx_engine::material::{Material, MaterialInstance};
use rx_engine::rand::{Rng, RngCore};
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

pub struct CameraMoveSystem;

pub struct ControlsSystem;


impl<'a> rx_engine::specs::System<'a> for CameraMoveSystem {
    type SystemData = (ReadStorage<'a, Camera>,
                       WriteStorage<'a, Position>,
                       WriteStorage<'a, Velocity>,
                       Read<'a, InputEventsRead>,
                       Read<'a, DeltaTime>);
    fn run(&mut self, (cam, mut pos, mut vel, input, delta): Self::SystemData) {
        let coof: f32 = 10f32;


        fn type_to_multiplier(val: f32, _type: &InputType) -> f32 {
            val * {
                match _type {
                    InputType::Begin => 1.,
                    InputType::End => 0.,
                    _ => 1.,
                }
            }
        }


        for (cam, mut pos, mut vel) in (&cam, &mut pos, &mut vel).join() {
            for e in &input.0 {
                match e {
                    InputEvent::Up(_type) => { vel.y = type_to_multiplier(coof, _type) }
                    InputEvent::Down(_type) => { vel.y = -type_to_multiplier(coof, _type) }
                    InputEvent::Left(_type) => { vel.x = -type_to_multiplier(coof, _type) }
                    InputEvent::Right(_type) => { vel.x = type_to_multiplier(coof, _type) }
                    InputEvent::Forward(_type) => { vel.z = -type_to_multiplier(coof, _type) }
                    InputEvent::Backward(_type) => { vel.z = type_to_multiplier(coof, _type) }
                    InputEvent::None => {}
                };
            }
        }
    }
}

impl<'a> rx_engine::specs::System<'a> for ControlsSystem {
    type SystemData = (Read<'a, PlatformEvents>,
                       Write<'a, InputEventsWrite>);

    fn run(&mut self, (platform, mut input): Self::SystemData) {
        fn action_to_type(action: &Action) -> InputType {
            match action {
                Action::Press => InputType::Begin,
                Action::Release => InputType::End,
                Action::Repeat => InputType::None,
            }
        }

        for e in &platform.0 {
            if let Event::Key(code, action) = e {
                let event = match code {
                    65 => InputEvent::Up(action_to_type(action)),       //space
                    54 => InputEvent::Down(action_to_type(action)),     //C
                    38 => InputEvent::Left(action_to_type(action)),     //A
                    40 => InputEvent::Right(action_to_type(action)),    //D
                    25 => InputEvent::Forward(action_to_type(action)),  //W
                    39 => InputEvent::Backward(action_to_type(action)), //S
                    _ => InputEvent::None
                };

                if let InputEvent::None = event {} else {
                    input.0.push(event);
                }
            }
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

            {
                let buffer = ctx.renderer_device.buffer();
            }

            let mut vertex_array: backend::VertexArray = ctx.renderer_device.vertex_array();
            let mut ib = ctx.renderer_device.index_buffer(&result.indices);
            let mut buffer = ctx.renderer_device.vertex_buffer();

            let uploader = rx_engine::mesh::MeshUploader{};

            vertex_array.set_index_buffer(ib);
            buffer.buffer_data_f32(&result.positions);
            buffer.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
            vertex_array.add_vertex_buffer(buffer);

            ctx.renderer.api().set_clear_color(0.3, 0.3, 0.9, 1_f32);

            let va_ptr: AssetPtr<backend::VertexArray> = ctx.asset_holder.storage_mut().put(vertex_array);


            let material: AssetPtr<Material> = ctx.asset_holder.storage_mut().put(rx_engine::material::Material::from_shader(ctx.renderer_device.shader(
                &relative_to_current_path(&vec!["client", "src", "test", "vert.glsl"]),
                &relative_to_current_path(&vec!["client", "src", "test", "frag.glsl"]),
                &BufferLayout::with(shared_types::FLOAT_3))));

            let mut instance: MaterialInstance = material.instance();
            instance.set_vec3("color_r", &glm::vec3(0.5, 0.5, 0.5));

            w.create_entity()
                .with(Camera::default())
                .with(Position {
                    x: 0.0,
                    y: 0.0,
                    z: 100.0,
                })
                .with(Rotation {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                })
                .with(Velocity {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                })
                .build();

            let mut rng = rx_engine::rand::thread_rng();

            for i in 0..100 {
                w.create_entity()
                    .with(Transformation {
                        mtx: glm::identity()
                    })
                    .with(Position {
                        x: rng.gen::<f32>() * 20.,
                        y: rng.gen::<f32>() * 20.,
                        z: rng.gen::<f32>() * 20.,
                    })
                    .with(Rotation {
                        x: rng.gen::<f32>() * 20.,
                        y: rng.gen::<f32>() * 20.,
                        z: rng.gen::<f32>() * 20.,
                    })
                    .with(Render {
                        va: va_ptr.clone(),
                        material: instance.clone(),
                    })
                    .build();
            }


            let d = d.with(ControlsSystem, "control_sys", &[]);
            let d = d.with(CameraMoveSystem, "cam_mov_sys", &[]);
            return (w, d);
        })),
    );
    dbg!();


    engine.run();
    println!("Bye!")
}
