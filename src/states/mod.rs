use specs::World;
#[path="../state_stack.rs"]
mod state_stack;
//use state_stack:*;
pub mod scene_state;

pub type MyStateStackOp = state_stack::StateStackOp<World, i32>;
pub type MyStateStack = state_stack::StateStack<World, i32>;
