use std::{
    sync::{
        mpsc,
        mpsc::Receiver,
        mpsc::Sender,
    }
};
use std::mem::size_of;

use interface::{CommandBuffer, RendererApi, RendererDevice};

use crate::loader::Loader;
use crate::Matrix4f;
use crate::utils::relative_to_current_path;

pub type DrawIndexed = (u32, u32, Matrix4f);

pub struct Renderer {
    api: backend::RendererApi,

    pipeline: backend::Pipeline,
    vertex: backend::Buffer,
    index: backend::Buffer,
    index_count: usize,
    uniform: backend::Buffer,
    instanced: backend::Buffer,

    pipeline_layout: backend::PipelineLayout,
    desc_set: backend::DescriptorSet,

    sender: Sender<DrawIndexed>,
    receiver: Receiver<DrawIndexed>,

    last_frame: Frame,
}

impl Renderer {
    pub fn new(api: backend::RendererApi, device: &backend::RendererDevice) -> Self {
        let mut path_buf = &relative_to_current_path(&vec!["client", "resources", "cube.obj"]);
        let mut loader = Loader;
        let result = loader.load_obj(path_buf);

        let static_mesh_buffer = device.create_buffer(interface::BufferDescriptor {
            size: 1024,
            usage: interface::Usage::Vertex,
        });
        let static_mesh_index_buffer = device.create_buffer(interface::BufferDescriptor {
            size: 1024,
            usage: interface::Usage::Index,
        });
        let uniform = device.create_buffer(interface::BufferDescriptor {
            size: 1024,
            usage: interface::Usage::Uniform,
        });
        let instanced = device.create_buffer(interface::BufferDescriptor {
            size: 16 * 4 * 30000,
            usage: interface::Usage::Vertex,
        });

        #[derive(Debug)]
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
        println!("{:?}", &vertexes);

        let b_ptr = device.map_buffer(&static_mesh_buffer);
        unsafe { std::ptr::copy(vertexes.as_ptr() as *mut u8, b_ptr, vertexes.len() * std::mem::size_of::<Vertex>()) }
        device.unmap_buffer(&static_mesh_buffer);

        let i_ptr = device.map_buffer(&static_mesh_index_buffer);
        unsafe { std::ptr::copy(result.indices.as_ptr() as *mut u8, i_ptr, result.indices.len() * size_of::<u32>()) }
        device.unmap_buffer(&static_mesh_index_buffer);


        let desc_set_layout = device.create_descriptor_set_layout(
            &[
                interface::DescriptorSetLayoutBinding {
                    binding: 0,
                    desc: interface::DescriptorType::UniformBuffer,
                }
            ]);

        let pipeline_layout = device.create_pipeline_layout(
            &desc_set_layout,
            vec![
                interface::PipelineLayoutHint {
                    location: 0,
                    hint: interface::LayoutHint::Name("Matricies"),
                }
            ]);

        let pipeline = {
            use interface;
            use std::mem::size_of;
            use std::fs;


            let shader_set = interface::ShaderSet {
                vertex: device.create_shader_mod(interface::ShaderModDescriptor {
                    stype: interface::ShaderType::Vertex,
                    source: fs::read_to_string(&relative_to_current_path(&vec!["client", "src", "test", "vert.glsl"])).expect(""),
                }),
                fragment: device.create_shader_mod(interface::ShaderModDescriptor {
                    stype: interface::ShaderType::Fragment,
                    source: fs::read_to_string(&relative_to_current_path(&vec!["client", "src", "test", "frag.glsl"])).expect(""),
                }),
            };

            let mut pipeline_desc = interface::PipelineDescriptor::new(
                interface::Primitive::Triangles,
                shader_set,
                &pipeline_layout,
            );

            pipeline_desc.push_vb(interface::VertexBufferDescriptor {
                binding: 0,
                stride: size_of::<Vertex>(),
            });

            pipeline_desc.push_vb(interface::VertexBufferDescriptor {
                binding: 1,
                stride: size_of::<[[f32; 4]; 4]>(),
            });

            pipeline_desc.push_attr(interface::AttributeDescriptor {
                binding: 0,
                location: 0,
                data: interface::VertexData {
                    offset: 0,
                    data_type: interface::DataType::Vec3f32,
                },
            });

            pipeline_desc.push_attr(interface::AttributeDescriptor {
                binding: 0,
                location: 1,
                data: interface::VertexData {
                    offset: size_of::<[f32; 3]>(),
                    data_type: interface::DataType::Vec2f32,
                },
            });

            pipeline_desc.push_attr(interface::AttributeDescriptor {
                binding: 0,
                location: 2,
                data: interface::VertexData {
                    offset: size_of::<[f32; 3]>() + size_of::<[f32; 2]>(),
                    data_type: interface::DataType::Vec3f32,
                },
            });

            pipeline_desc.push_attr(interface::AttributeDescriptor {
                binding: 1,
                location: 3,
                data: interface::VertexData {
                    offset: 0,
                    data_type: interface::DataType::Mat4f32,
                },
            });

            device.create_pipeline(pipeline_desc)
        };

        let desc_set = device.allocate_descriptor_set(&desc_set_layout);

        device.write_descriptor_set(interface::DescriptorSetWrite {
            set: &desc_set,
            binding: 0,
            descriptor: interface::Descriptor::Buffer(&uniform),
        });


        let (s, r) = mpsc::channel();
        Renderer {
            api,
            sender: s.clone(),
            pipeline,
            vertex: static_mesh_buffer,
            index: static_mesh_index_buffer,
            uniform: uniform,
            pipeline_layout,
            desc_set,
            index_count: result.indices.len(),
            receiver: r,
            last_frame: Frame {
                queue: s.clone(),
                view: glm::identity(),
                projection: glm::identity(),
            },
            instanced,
        }
    }
}

pub struct Frame {
    queue: Sender<DrawIndexed>,
    view: Matrix4f,
    projection: Matrix4f,
}

impl Frame {
    pub fn queue(&self) -> Sender<DrawIndexed> {
        self.queue.clone()
    }

    pub fn set_view_matrix(&mut self, mtx: Matrix4f) {
        self.view = mtx
    }

    pub fn set_projection_matrix(&mut self, mtx: Matrix4f) {
        self.projection = mtx
    }
}

impl Renderer {
    pub fn submit(&mut self, cmd: DrawIndexed) {
        self.sender.send(cmd);
    }

    pub fn process(&self, device: &backend::RendererDevice, frame: &mut Frame) {
        let mut cmd_buffer = device.create_cmd_buffer();
        let u_ptr = device.map_buffer(&self.uniform);

        cmd_buffer.clear_screen((0.5, 0.5, 0.5, 1.));
        cmd_buffer.prepare_pipeline(&self.pipeline);
        cmd_buffer.bind_descriptor_set(&self.pipeline_layout, &self.desc_set);

        cmd_buffer.bind_vertex_buffer(0, &self.vertex);
        cmd_buffer.bind_vertex_buffer(1, &self.instanced);
        cmd_buffer.bind_index_buffer(&self.index);

        unsafe {
            std::ptr::copy(frame.view.as_slice().as_ptr() as *mut u8, u_ptr, 1 * 16 * size_of::<u32>());
            std::ptr::copy(frame.projection.as_slice().as_ptr() as *mut u8, u_ptr.offset(1 * 16 * 4), 1 * 16 * size_of::<u32>());
        }

        let vp = frame.projection * frame.view;
        let mapped = device.map_buffer(&self.instanced);

        let mut n = 0;
        for (i, cmd) in self.receiver.try_iter().enumerate() {
//            let va: &backend::VertexArray = ctx.storage().get_ref(&cmd.0).unwrap();
//            let instance = cmd.1;
//            let material: &Material = ctx.storage().get_ref(instance.material()).unwrap();
//
//            material.bind();
//            instance.prepare(material);
//            let shader = material.shader();
//
//            shader.load_mat4("r_view", frame.view.as_slice());
//            shader.load_mat4("r_projection", frame.projection.as_slice());
//            shader.load_mat4("r_vp", (frame.projection * frame.view).as_slice());
//            shader.load_mat4("r_transformation", cmd.2.as_slice());
//            self.api.draw_indexed(va);
//            shader.unbind();
            unsafe {
                std::ptr::copy((vp * cmd.2).as_slice().as_ptr() as *mut u8, mapped.offset(((i as u32) * 16 * 4) as isize), 1 * 16 * size_of::<u32>());
            };
            n = i;
        }

        cmd_buffer.draw_indexed(self.index_count as u32, 0, (n + 1) as u32);

        device.unmap_buffer(&self.uniform);
        device.unmap_buffer(&self.instanced);
        device.execute(cmd_buffer)
    }

    pub fn viewport(&self, w: i32, h: i32) {
        self.api.viewport(w, h);
    }

    pub fn start(&mut self) -> Frame {
        self.api.clear_color();
        Frame {
            queue: self.sender.clone(),
            view: self.last_frame.view,
            projection: self.last_frame.projection,
        }
    }

    pub fn process_frame(&mut self, device: &backend::RendererDevice, frame: &mut Frame) {
        self.process(device, frame);
    }

    pub fn end(&mut self, frame: Frame) {
//        self.api.swap_buffer();
        self.last_frame = frame;
    }

    pub fn get_submitter(&self) -> Sender<DrawIndexed> {
        self.sender.clone()
    }

    pub fn api(&self) -> &backend::RendererApi {
        &self.api
    }
}