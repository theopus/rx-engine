extern crate rx_engine;

use rx_engine::ecs::layer::EcsLayerBuilder;
use rx_engine::interface::WindowConfig;
use rx_engine::specs;

mod sandbox_layer;


pub struct EmptySystem;

impl<'a> rx_engine::specs::System<'a> for EmptySystem {
    type SystemData = ();
    fn run(&mut self, data: Self::SystemData) {
        println!("Im retard");
    }
}


fn main() {
    let mut engine = rx_engine::run::build_engine(
        WindowConfig { width: 600, height: 400 },
        EcsLayerBuilder::new(Box::new(|w, d| {
            let d = d.with(EmptySystem, "retard_system", &[]);
            (w,d)
        }))
    );
    engine.add_layer_builder(sandbox_layer::SandboxLayerBuilder);
    engine.run();
    println!("Bye!")
}
