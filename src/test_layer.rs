use ggez;
use specs::{self,Join,World};
//mod systems;
//mod layer_stack;

use super::layer_stack::*;
use super::systems::TestSystem;

pub type MyLayerStackOp = LayerStackOp<World, i32>;
pub type MyLayerStack = LayerStack<World, i32>;


pub struct TestLayer{
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl TestLayer {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let dispatcher = Self::register_systems();
    TestLayer {
        dispatcher,
        }   
    }
    fn register_systems() -> specs::Dispatcher<'static, 'static>{
        specs::DispatcherBuilder::new()
            .with(TestSystem, "sys_test", &[])
            .build()
    }
}

impl Layer<World, i32> for TestLayer {
    fn update(&mut self, world_ctx: &mut World) -> MyLayerStackOp {
        //self.dispatcher.dispatch(&mut world_ctx.specs_world.res);
        //println!("Updated TestLayer!");
        LayerStackOp::None
    }

    fn draw(&mut self, world_ctx: &mut World, ggez_ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }
    fn input (&mut self, world_ctx: &mut World, input: i32, active: bool) {
        ()
    }
}

