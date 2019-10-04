extern crate backend_opengl as backend;

pub use backend_opengl::backend_api as api;

pub type VertexArray = <backend::Backend as api::Backend>::VertexArray;
pub type VertexBuffer = <backend::Backend as api::Backend>::VertexBuffer;
pub type IndexBuffer = <backend::Backend as api::Backend>::IndexBuffer;
pub type Shader = <backend::Backend as api::Backend>::Shader;
pub type RendererApi = <backend::Backend as api::Backend>::RendererApi;
pub type RendererDevice = <backend::Backend as api::Backend>::RendererDevice;
pub type PlatformManager = <backend::Backend as api::Backend>::PlatformManager;



