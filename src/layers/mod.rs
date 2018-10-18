use specs::{self,Join,World};

pub mod test_layer;
use layer_stack::*;
//use test_layer;

pub type MyLayerStackOp = LayerStackOp<World, i32>;
pub type MyLayerStack = LayerStack<World, i32>;


