extern crate backend_opengl as backend;

pub use backend_opengl::backend_interface as interface;

pub type VertexArray = <backend::Backend as interface::Backend>::VertexArray;
pub type VertexBuffer = <backend::Backend as interface::Backend>::VertexBuffer;
pub type IndexBuffer = <backend::Backend as interface::Backend>::IndexBuffer;
pub type Shader = <backend::Backend as interface::Backend>::Shader;
pub type RendererApi = <backend::Backend as interface::Backend>::RendererApi;
pub type RendererConstructor = <backend::Backend as interface::Backend>::RendererConstructor;
pub type PlatformManager = <backend::Backend as interface::Backend>::PlatformManager;



