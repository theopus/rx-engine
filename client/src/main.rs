extern crate rx_engine;

use std::path::Path;

use rx_engine::loader::Loader;
use rx_engine::backend::interface::WindowConfig;
use rx_engine::run::MutLayerBuilder;
use rx_engine::run::PushLayer;
use rx_engine::utils::relative_path;
use rx_engine::utils::relative_to_current_path;
use core::borrow::BorrowMut;
use std::env;

mod sandbox_layer;


fn main() {

    let mut engine = rx_engine::run::build_engine(WindowConfig { width: 600, height: 400 });
    engine.push_layer(Box::new(sandbox_layer::get_layer) as MutLayerBuilder);
    engine.run();
    println!("Bye!")
}
