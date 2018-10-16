use ggez;
use specs::{self,Join,World};
#[path="../state_stack.rs"]
mod state_stack;

use states::*;
use systems::*;
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

impl state_stack::State<World, i32> for TestState {
    fn update(&mut self, world_ctx: &mut World) -> MyStateStackOp {
        self.dispatcher.dispatch(&mut world_ctx.specs_world.res);
        state_stack::StateStackOp::None
    }
}
