extern crate rx_engine;

use rx_engine::platform::{create_pm, WindowConfig};
use rx_engine::render::{BufferLayout, RendererType, VertexArray, VertexBuffer};
use rx_engine::render::shared_types;


fn main() {
    let platform_manager = create_pm(WindowConfig { width: 600, height: 400 });
    let (api, constructor) = platform_manager.create_renderer(RendererType::OpenGL);

    let mut vertex_array: Box<VertexArray> = constructor.vertex_array();
    let mut ib = constructor.index_buffer(&[0, 1, 3, 3, 1, 2]);
    let mut vb: Box<VertexBuffer> = constructor.vertex_buffer();

    vertex_array.set_index_buffer(ib);

    vb.buffer_data_f32(&[
        -0.5_f32, 0.5_f32, 0_f32, // 0

        -0.5_f32, -0.5_f32, 0_f32, // 1

        0.5_f32, -0.5_f32, 0_f32, // 2

        0.5_f32, 0.5_f32, 0_f32, //3
    ]);
    vb.set_buffer_layout(BufferLayout::with(shared_types::FLOAT_3));
    vertex_array.add_vertex_buffer(vb);

    api.set_clear_color(0.3, 0.3, 0.9, 1_f32);


    while !platform_manager.should_close() {
        api.clear_color();
        platform_manager.process_events();
        api.draw_indexed(&vertex_array);
        api.swap_buffer();
    }
}
