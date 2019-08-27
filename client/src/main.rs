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
use rx_engine::material::{Material, MaterialInstance};
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


            return (w, d);
        })),
    );
    dbg!();


    engine.run();
    println!("Bye!")
}
