use crate::ecs::DeltaTime;
use specs::Read;
use crate::run::MutLayerBuilder;
use crate::run::MutLayer;

use specs::{
    Dispatcher,
    World,
    WorldExt,
    System,
};
use crate::backend::PlatformManager;
use crate::render::Renderer;

pub struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = (Read<'a, DeltaTime>);
    fn run(&mut self, data: Self::SystemData) {
       //nothing
    }
}


pub struct EcsLayer<'a> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'a>,
}

impl<'a> EcsLayer<'a> {
    pub fn new() -> Self {
        let mut world: specs::World = specs::WorldExt::new();
        let dispatcher = specs::DispatcherBuilder::new()
            .with(EmptySystem, "empty_system", &[])
            .build();
        world.add_resource(DeltaTime(0f64));

        EcsLayer { world, dispatcher }
    }

    pub fn builder() -> MutLayerBuilder {
        let builder: MutLayerBuilder = Box::new(|a, b| {
            Box::new(EcsLayer::new())
        });
        builder
    }
}

impl<'a> MutLayer for EcsLayer<'a> {
    fn on_update(&mut self, delta: f64, renderer_api: &mut Renderer, platform_manager: &mut PlatformManager) {
        self.dispatcher.dispatch(&self.world);
        let mut delta_resource = self.world.write_resource::<DeltaTime>();
        *delta_resource = DeltaTime(delta);
    }
}