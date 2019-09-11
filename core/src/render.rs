use std::{
    sync::{
        mpsc,
        mpsc::Receiver,
        mpsc::Sender,
    }
};
use std::mem::size_of;

use interface::{RendererApi, RendererDevice, Shader};

use crate::asset::{AssetHolder, AssetPtr};
use crate::loader::Loader;
use crate::material::{Material, MaterialInstance};
use crate::Matrix4f;
use crate::utils::relative_to_current_path;

pub type DrawIndexed = (AssetPtr<backend::VertexArray>, MaterialInstance, Matrix4f);

pub struct Renderer {
    api: backend::RendererApi,

    pipeline: backend::Pipeline,
    vertex: backend::Buffer,
    index: backend::Buffer,

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

        let b_ptr = device.map_buffer(&static_mesh_buffer);
        unsafe { std::ptr::copy(vertexes.as_ptr() as *mut u8, b_ptr, vertexes.len() * std::mem::size_of::<Vertex>()) }
        device.unmap_buffer(&static_mesh_buffer);

        let i_ptr = device.map_buffer(&static_mesh_index_buffer);
        unsafe { std::ptr::copy(result.indices.as_ptr() as *mut u8, i_ptr, result.indices.len() * size_of::<u32>()) }
        device.unmap_buffer(&static_mesh_index_buffer);

        let pipeline = {
            use interface;
            use std::mem::size_of;
            use std::fs;

            let desc_set_layout = device.create_descriptor_set_layout(&[interface::DescriptorSetLayoutBinding {
                location: 0,
                desc: interface::DescriptorType::UniformBuffer,
            }]);

            let pipeline_layout = device.create_pipeline_layout(desc_set_layout);


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
                pipeline_layout,
            );

            pipeline_desc.push_vb(interface::VertexBufferDescriptor {
                binding: 0,
                stride: size_of::<[f32; 3]>(),
            });

            pipeline_desc.push_attr(interface::AttributeDescriptor {
                binding: 0,
                location: 0,
                data: interface::VertexData {
                    offset: size_of::<[f32; 3]>() * 0,
                    data_type: interface::DataType::Vec3f32,
                },
            });

            device.create_pipeline(pipeline_desc)
        };

        let (s, r) = mpsc::channel();
        Renderer {
            api,
            sender: s.clone(),
            pipeline,
            vertex: static_mesh_buffer,
            index: static_mesh_index_buffer,
            receiver: r,
            last_frame: Frame {
                queue: s.clone(),
                view: glm::identity(),
                projection: glm::identity(),
            },
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

    pub fn process(&self, device: &backend::RendererDevice, ctx: &mut AssetHolder, frame: &mut Frame) {
        for cmd in self.receiver.try_iter() {
            let va: &backend::VertexArray = ctx.storage().get_ref(&cmd.0).unwrap();
            let instance = cmd.1;
            let material: &Material = ctx.storage().get_ref(instance.material()).unwrap();

            material.bind();
            instance.prepare(material);
            let shader = material.shader();

            shader.load_mat4("r_view", frame.view.as_slice());
            shader.load_mat4("r_projection", frame.projection.as_slice());
            shader.load_mat4("r_vp", (frame.projection * frame.view).as_slice());
            shader.load_mat4("r_transformation", cmd.2.as_slice());
            self.api.draw_indexed(va);
            shader.unbind();
        }
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

    pub fn process_frame(&mut self, device: &backend::RendererDevice, frame: &mut Frame, ctx: &mut AssetHolder) {
        self.process(device, ctx, frame);
    }

    pub fn end(&mut self, frame: Frame) {
        self.api.swap_buffer();
        self.last_frame = frame;
    }

    pub fn get_submitter(&self) -> Sender<DrawIndexed> {
        self.sender.clone()
    }

    pub fn api(&self) -> &backend::RendererApi {
        &self.api
    }
}