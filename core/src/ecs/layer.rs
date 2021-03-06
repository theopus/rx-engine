use std::sync::mpsc::Sender;

use specs::{Dispatcher, DispatcherBuilder, System, World, WorldExt};
use specs::Join;
use specs::Read;
use specs::ReadStorage;
use specs::WriteStorage;

use crate::backend::{PlatformManager, RendererDevice};
use crate::ecs::{ActiveCamera, DeltaTime, InputEventsRead, InputEventsWrite, PlatformEvents};
use crate::ecs::components::{Camera, Position, Render, Rotation, Transformation, Velocity};
use crate::ecs::system::{CameraSystem, MoveSystem, TransformationSystem};
use crate::api::Event;
use crate::render::DrawIndexed;
use crate::render::Renderer;
use crate::run::{EngineContext, FrameContext, Layer, LayerBuilder};

pub struct EmptySystem;

impl<'a> System<'a> for EmptySystem {
    type SystemData = (Read<'a, DeltaTime>);
    fn run(&mut self, data: Self::SystemData) {
        //nothing
    }
}

pub struct RenderSystem {
    sender: Sender<DrawIndexed>
}

impl RenderSystem {
    pub fn new(sender: Sender<DrawIndexed>) -> Self {
        RenderSystem { sender }
    }
}

impl<'a, 'd> System<'a> for RenderSystem {
    type SystemData = (ReadStorage<'a, Transformation>,
                       WriteStorage<'a, Render>);

    fn run(&mut self, (transformation, mut render): Self::SystemData) {
        for (transformation, mut render) in (&transformation, &mut render).join() {
            self.sender.send((render.va.clone(), render.material.clone(), transformation.mtx));
        }
    }
}

pub struct EcsLayer<'a> {
    world: specs::World,
    dispatcher: specs::Dispatcher<'a, 'a>,
}

impl<'a> EcsLayer<'a> {
    pub fn new(sender: Sender<DrawIndexed>, init: &EcsInit<'a>, ctx: &mut EngineContext) -> Self {
        let mut world: specs::World = specs::WorldExt::new();
        world.register::<Position>();
        world.register::<Rotation>();
        world.register::<Transformation>();
        world.register::<Camera>();
        world.register::<Render>();
        world.register::<Velocity>();

        world.insert(DeltaTime(0f64));
        world.insert(PlatformEvents(Vec::new()));
        world.insert(InputEventsRead(Vec::new()));
        world.insert(InputEventsWrite(Vec::new()));
        world.insert(ActiveCamera::default());

        let render_system: RenderSystem = RenderSystem::new(sender);
        let dispatcher = specs::DispatcherBuilder::new()
            .with(EmptySystem, "empty_system", &[])
            .with(TransformationSystem, "tsm_system", &[])
            .with(MoveSystem, "move_system", &[])
            .with(CameraSystem, "camera_system", &[])
            .with_thread_local(render_system);

        let ctx: &mut EngineContext = ctx;
        let (world, dispatcher) = init(world, dispatcher, ctx);
        let dispatcher = dispatcher.build();


        EcsLayer { world, dispatcher }
    }
}


pub type EcsInit<'a> = Box<fn(specs::World, specs::DispatcherBuilder<'a, 'a>, ctx: &mut EngineContext) -> (specs::World, specs::DispatcherBuilder<'a, 'a>)>;

pub struct EcsLayerBuilder<'a> {
    ecs_builder_fn: EcsInit<'a>
}

impl<'a> EcsLayerBuilder<'a> {
    pub fn new(ecs_builder_fn: EcsInit<'a>) -> EcsLayerBuilder<'a> {
        EcsLayerBuilder { ecs_builder_fn }
    }
}

impl<'a> Default for EcsLayerBuilder<'a> {
    fn default() -> Self {
        let f: EcsInit<'a> = Box::new(|world, dispatcher, ctx| {
            return (world, dispatcher);
        });
        EcsLayerBuilder { ecs_builder_fn: f }
    }
}

impl<'l> LayerBuilder<'l> for EcsLayerBuilder<'l> {
    fn build(&self, ctx: &mut EngineContext) -> Box<dyn Layer + 'l> {
        Box::new(
            EcsLayer::new(ctx.renderer.get_submitter(),
                          &self.ecs_builder_fn,
                          ctx))
    }
}

impl<'a> Layer for EcsLayer<'a> {
    fn on_update(&mut self, frame: &mut FrameContext, ctx: &mut EngineContext) {
        {
            let mut delta_resource = self.world.write_resource::<DeltaTime>();
            let mut events_resource = self.world.write_resource::<PlatformEvents>();

            events_resource.0.clear();
            for e in &frame.events {
                events_resource.0.push((*e).clone());
            }
            *delta_resource = DeltaTime(frame.elapsed);

            let mut write_e = self.world.write_resource::<InputEventsWrite>();
            let mut read_e = self.world.write_resource::<InputEventsRead>();
            read_e.0.clear();
            read_e.0.append(&mut write_e.0);
        }

        self.dispatcher.dispatch(&self.world);

        {
            let mut camera = self.world.read_resource::<ActiveCamera>();
            frame.frame.set_view_matrix(camera.view_mtx);
            frame.frame.set_projection_matrix(camera.proj_mtx);
        }
    }
}