use ggez;
use specs::{self,Join,World};
//mod systems;
//mod state_stack;

use super::state_stack::*;
use super::systems::TestSystem;

pub type MyStateStackOp = StateStackOp<World, i32>;
pub type MyStateStack = StateStack<World, i32>;


pub struct TestState{
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl TestState {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let dispatcher = Self::register_systems();
    TestState {
        dispatcher,
        }   
    }
    fn register_systems() -> specs::Dispatcher<'static, 'static>{
        specs::DispatcherBuilder::new()
            .with(TestSystem, "sys_test", &[])
            .build()
    }
}

impl State<World, i32> for TestState {
    fn update(&mut self, world_ctx: &mut World) -> MyStateStackOp {
        //self.dispatcher.dispatch(&mut world_ctx.specs_world.res);
        StateStackOp::None
    }

    fn draw(&mut self, world_ctx: &mut World, ggez_ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }
    fn input (&mut self, world_ctx: &mut World, input: i32, active: bool) {
        ()
    }
}

