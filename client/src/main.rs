extern crate rx_engine;

use specs::{Builder, Join, Read, ReadStorage, WorldExt, Write, WriteStorage};

use rx_engine::{Matrix4f, specs};
use rx_engine::asset::AssetPtr;
use rx_engine::backend;
use rx_engine::ecs::{ActiveCamera, DeltaTime, InputEvent, InputEventsRead, InputEventsWrite, PlatformEvents};
use rx_engine::ecs::components::{Camera, Position, Render, Rotation, Transformation};
use rx_engine::ecs::layer::EcsLayerBuilder;
use rx_engine::glm;
use rx_engine::interface::{Action, BufferLayout, Event, shared_types, VertexArray, VertexBuffer, WindowConfig};
use rx_engine::interface::{RendererApi, RendererConstructor};
use rx_engine::loader::Loader;
use rx_engine::material::{Material, MaterialInstance};
use rx_engine::utils::relative_to_current_path;

pub struct EmptySystem;

pub struct ControlsSystem;

impl<'a> rx_engine::specs::System<'a> for EmptySystem {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Rotation>,
                       WriteStorage<'a, Transformation>);
    fn run(&mut self, data: Self::SystemData) {
        println!("Im retard");
    }
}

pub struct CameraMoveSystem;


impl<'a> rx_engine::specs::System<'a> for CameraMoveSystem {
    type SystemData = (ReadStorage<'a, Camera>,
                       WriteStorage<'a, Position>,
                       Read<'a, InputEventsRead>);
    fn run(&mut self, (cam, mut pos, input): Self::SystemData) {
        let coof = 0.1;

        for (cam, mut pos) in (&cam, &mut pos).join() {
            for e in &input.0 {
                match e {
                    InputEvent::Up => {pos.y += coof},
                    InputEvent::Down => {pos.y -= coof},
                    InputEvent::Left => {pos.x -= coof},
                    InputEvent::Right => {pos.x += coof},
                    InputEvent::Forward => {pos.z -= coof},
                    InputEvent::Backward => {pos.z += coof},
                    InputEvent::None => {},
                };
            }
        }
    }
}

impl<'a> rx_engine::specs::System<'a> for ControlsSystem {
    type SystemData = (Read<'a, PlatformEvents>,
                       Write<'a, InputEventsWrite>);

    fn run(&mut self, (platform, mut input): Self::SystemData) {
        for e in &platform.0 {
            if let Event::Key(code, action) = e {
                let event = match code {
                    65 => InputEvent::Up,       //space
                    54 => InputEvent::Down,     //C
                    38 => InputEvent::Left,     //A
                    40 => InputEvent::Right,    //D
                    25 => InputEvent::Forward,  //W
                    39 => InputEvent::Backward, //S
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

            let mut vertex_array: backend::VertexArray = ctx.renderer_constructor.vertex_array();
            let mut ib = ctx.renderer_constructor.index_buffer(&result.indices);
            let mut vb: Box<backend::VertexBuffer> = Box::new(ctx.renderer_constructor.vertex_buffer());

            vertex_array.set_index_buffer(ib);
            vb.buffer_data_f32(&result.positions);
            vb.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
            vertex_array.add_vertex_buffer(*vb);

            ctx.renderer.api().set_clear_color(0.3, 0.3, 0.9, 1_f32);

            let va_ptr: AssetPtr<backend::VertexArray> = ctx.asset_holder.storage_mut().put(vertex_array);


            let material: AssetPtr<Material> = ctx.asset_holder.storage_mut().put(rx_engine::material::Material::from_shader(ctx.renderer_constructor.shader(
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
                    material: instance,
                })
                .build();


            let d = d.with(ControlsSystem, "control_sys", &[]);
            let d = d.with(CameraMoveSystem, "cam_mov_sys", &[]);
            return (w, d);
        })),
    );
    dbg!();


    engine.run();
    println!("Bye!")
}
