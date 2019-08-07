extern crate rx_engine;

use rx_engine::interface::WindowConfig;

mod sandbox_layer;


fn main() {
    let mut engine = rx_engine::run::build_engine(WindowConfig { width: 600, height: 400 });
    engine.add_layer_builder(Box::new(sandbox_layer::SandboxLayerBuilder));
    engine.run();
    println!("Bye!")
}
