use std::sync::mpsc::Sender;

use specs::{
    Dispatcher,
    System,
    World,
    WorldExt,
};
use specs::Join;
use specs::Read;
use specs::ReadStorage;
use specs::WriteStorage;

use crate::backend::{PlatformManager, RendererConstructor};
use crate::ecs::components::{Position, Rotation, Transformation};
use crate::ecs::DeltaTime;
use crate::render::DrawIndexed;
use crate::render::Renderer;
use crate::run::{EngineContext, Layer, LayerBuilder};

pub struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = (Read<'a, DeltaTime>);
    fn run(&mut self, data: Self::SystemData) {
        //nothing
    }
}

pub struct RenderSystem<'d> {
    sender: Sender<DrawIndexed<'d>>
}

impl<'d> RenderSystem<'d> {
    pub fn new(sender: Sender<DrawIndexed<'d>>) -> Self {
        RenderSystem { sender }
    }
}

impl<'a, 'd> System<'a> for RenderSystem<'d> {
    type SystemData = (ReadStorage<'a, Transformation>);

    fn run(&mut self, transformation: Self::SystemData) {}
}

struct TransformationSystem;


impl<'a> System<'a> for TransformationSystem {
    type SystemData = (ReadStorage<'a, Position>,
                       ReadStorage<'a, Rotation>,
                       WriteStorage<'a, Transformation>);

    fn run(&mut self, (pos, rot, mut tsm): Self::SystemData) {
        for (pos, rot, tsm) in (&pos, &rot, &mut tsm).join() {
            tsm.mtx = {
                let mut mtx = glm::identity();
                glm::rotate(&mut mtx, rot.x, &glm::vec3(1., 0., 0.)) *
                    glm::rotate(&mut mtx, rot.y, &glm::vec3(0., 1., 0.)) *
                    glm::rotate(&mut mtx, rot.z, &glm::vec3(0., 0., 1.)) *
                    glm::translate(&mut mtx, &glm::vec3(pos.x, pos.y, pos.z))
            };
        }
    }
}


pub struct EcsLayer<'a> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'a>,
}

impl<'a> EcsLayer<'a> {
    pub fn new(sender: Sender<DrawIndexed<'a>>) -> Self {
        let mut world: specs::World = specs::WorldExt::new();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Transformation>();

        let render_system: RenderSystem<'a> = RenderSystem::new(sender);
        let dispatcher = specs::DispatcherBuilder::new()
            .with(EmptySystem, "empty_system", &[])
            .with(TransformationSystem, "tsm_system", &[])
            .with_thread_local(render_system)
            .build();
        world.insert(DeltaTime(0f64));

        EcsLayer { world, dispatcher }
    }
}

pub struct EcsLayerBuilder;

impl<'l> LayerBuilder<'l> for EcsLayerBuilder {
    fn build(&self, ctx: &mut EngineContext<'l>) -> Box<dyn Layer + 'l> {
        Box::new(EcsLayer::new(ctx.renderer.get_submitter()))
    }
}

impl<'a> Layer for EcsLayer<'a> {
    fn on_update(&mut self, delta: f64, ctx: &mut EngineContext) {
        {
            let mut delta_resource = self.world.write_resource::<DeltaTime>();
            *delta_resource = DeltaTime(delta);
            //dropping resource borrow
        }


        self.dispatcher.dispatch(&self.world);
    }
}