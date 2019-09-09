extern crate rx_engine;

use specs::{Builder, Join, Read, ReadStorage, WorldExt, Write, WriteStorage};

use rx_engine::{
    asset::AssetPtr,
    backend,
    ecs::{
        ActiveCamera,
        components::{Camera, Position, Render, Rotation, Transformation, Velocity},
        DeltaTime,
        InputEvent,
        InputEventsRead,
        InputEventsWrite,
        InputType,
        layer::EcsLayerBuilder,
        PlatformEvents,
    },
    glm,
    interface::{
        Action,
        BufferLayout,
        Event,
        RendererApi,
        RendererDevice,
        shared_types,
        VertexArray,
        VertexBuffer,
        WindowConfig,
    },
    loader::Loader,
    material::{Material, MaterialInstance},
    Matrix4f,
    rand::{Rng, RngCore},
    specs,
    utils::relative_to_current_path,
};

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

        use rx_engine::specs::Join;
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
    let mut engine: rx_engine::run::RxEngine = rx_engine::run::build_engine(
        WindowConfig { width: 600, height: 400 },
        EcsLayerBuilder::new(Box::new(|mut w, d, ctx| {
            let mut path_buf = &relative_to_current_path(&vec!["client", "resources", "cube.obj"]);
            let mut loader = Loader;
            let result = loader.load_obj(path_buf);

            {
                let static_mesh_buffer = ctx.renderer_device.create_buffer(rx_engine::interface::BufferDescriptor {
                    size: 1024,
                    usage: rx_engine::interface::Usage::Vertex,
                });



                struct Vertex {
                    position: [f32; 3],
                    uv: [f32; 2],
                    normal: [f32; 3],
                }

                impl Vertex {
                    pub fn from_pos_norm(positions: &Vec<f32>, normals: &Vec<f32>) -> Vec<Vertex> {
                        assert_eq!(positions.len(), normals.len(), "different size of positions and normals");
                        positions
                            .chunks_exact(3)
                            .zip(normals.chunks_exact(3))
                            .map(|(p, n): (&[f32], &[f32])| {
                                let p = p.to_vec();
                                let n = n.to_vec();
                                Vertex {
                                    position: [
                                        *(p.get(0).unwrap()),
                                        *(p.get(1).unwrap()),
                                        *(p.get(2).unwrap()),
                                    ],
                                    uv: [0., 0.],
                                    normal: [
                                        *(n.get(0).unwrap()),
                                        *(n.get(1).unwrap()),
                                        *(n.get(2).unwrap()),
                                    ],
                                }
                            }).collect::<Vec<Vertex>>()
                    }
                };
                let vertexes = Vertex::from_pos_norm(&result.positions, &result.normals);

                let b_ptr = ctx.renderer_device.map_buffer(&static_mesh_buffer);
                unsafe { std::ptr::copy(vertexes.as_ptr() as *mut u8, b_ptr, vertexes.len() * std::mem::size_of::<Vertex>()) }
                let vert_from_gl = unsafe { std::slice::from_raw_parts(b_ptr as *mut f32, std::mem::size_of::<Vertex>() * vertexes.len() / std::mem::size_of::<f32>()) };
                println!("{:?}", vert_from_gl);

                ctx.renderer_device.unmap_buffer(&static_mesh_buffer);

                use rx_engine::interface;

                let desc_set_layout = ctx.renderer_device.create_descriptor_set_layout(&[interface::DescriptorSetLayoutBinding {
                    location: 0,
                    desc: interface::DescriptorType::UniformBuffer,
                }]);

                let pipeline_layout = ctx.renderer_device.create_pipeline_layout(desc_set_layout);


                let pipeline = {
                    use rx_engine::interface;
                    use std::mem::size_of;
                    use std::fs;


                    let shader_set = interface::ShaderSet {
                        vertex: ctx.renderer_device.create_shader_mod(interface::ShaderModDescriptor {
                            stype: interface::ShaderType::Vertex,
                            source: fs::read_to_string(&relative_to_current_path(&vec!["client", "src", "ubershader", "vert.glsl"])).expect(""),
                        }),
                        fragment: ctx.renderer_device.create_shader_mod(interface::ShaderModDescriptor {
                            stype: interface::ShaderType::Fragment,
                            source: fs::read_to_string(&relative_to_current_path(&vec!["client", "src", "ubershader", "frag.glsl"])).expect(""),
                        }),
                    };

                    let mut pipeline_desc = interface::PipelineDescriptor::new(
                        interface::Primitive::Triangles,
                        shader_set,
                        pipeline_layout,
                    );

                    pipeline_desc.push_vb(interface::VertexBufferDescriptor {
                        binding: 0,
                        stride: size_of::<Vertex>(),
                    });

                    pipeline_desc.push_attr(interface::AttributeDescriptor {
                        binding: 0,
                        location: 0,
                        data: interface::VertexData {
                            offset: size_of::<[f32; 3]>() * 0,
                            data_type: interface::DataType::Vec3f32,
                        },
                    });

                    pipeline_desc.push_attr(interface::AttributeDescriptor {
                        binding: 0,
                        location: 1,
                        data: interface::VertexData {
                            offset: size_of::<[f32; 2]>() * 1,
                            data_type: interface::DataType::Vec2f32,
                        },
                    });

                    pipeline_desc.push_attr(interface::AttributeDescriptor {
                        binding: 0,
                        location: 2,
                        data: interface::VertexData {
                            offset: size_of::<[f32; 3]>() * 2,
                            data_type: interface::DataType::Vec3f32,
                        },
                    });

                    ctx.renderer_device.create_pipeline(pipeline_desc)
                };
            }

            let mut vertex_array: backend::VertexArray = ctx.renderer_device.vertex_array();
            let mut ib = ctx.renderer_device.index_buffer(&result.indices);
            let mut buffer = ctx.renderer_device.vertex_buffer();

            let uploader = rx_engine::mesh::MeshUploader {};

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
