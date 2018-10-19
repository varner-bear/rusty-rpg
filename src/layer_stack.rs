
//! The Layer System for the RPG Game Engine.
//! The system uses a pushdown automata to maintain the current gamee layer
//! along with any GUI's rendered over the game display.

/* Goals
 * 1) Imlement the PDA with a basic layer structure
 * 2) Build a test layer to display a background image
 * 3) Start a new layer with a key press (stack push)
 * 4) Return to the first layer with a key press (stack pop)
*/

use ggez;
use warmy;
use resources;

/// Layer Trait to be implemented for our various game layers in terms of a
/// common context and input type
/// Defines functions to update and render the layer and handle imput events
pub trait Layer<C, I> {
    /// Updates the current layer which could involve modifying the LayerStack
    fn update(&mut self, world_ctx: &mut C) -> LayerStackOp<C, I>;
    /// Renders the Layer using to GGEZ
    fn draw(&mut self, world_ctx: &mut C, ggex_ctx: &mut ggez::Context) -> 
        ggez::GameResult<()>;
    /// Handles an input event for an active layer
    fn input(&mut self, world_ctx: &mut C, input: I, active: bool);
    /// Allows for drawing multiple layers for displaying layered GUI
    /// Defaults to false until implemented
    fn draw_recursive(&self) -> bool {false} 
    fn is_active(&self) -> bool {true}

}
/// An Layer Stack Operation Command
/// Pushes a new layer onto the stack, pops one off or replaces the current
/// layer with a pup and subsequent push
pub enum LayerStackOp<C, I> {
    None,
    Pop,
    Push(Box<Layer<C, I>>),
    Swap(Box<Layer<C, I>>),
}

impl<C, I> LayerStackOp<C, I> {
    /// Shortcut for calling Scene Stack Operations (Swap)
    pub fn swap<S>(layer: S) -> Self
    where
        S: Layer<C, I> + 'static,
    {
        LayerStackOp::Swap(Box::new(layer))   
    }

    /// Shortcut for calling Scene Stack Operations (Push)
    pub fn push<S> (layer: S) -> Self
    where   
        S: Layer<C, I> + 'static,
    {
        LayerStackOp::Push(Box::new(layer))
    }
}

pub struct LayerStack<C, I> {
    pub world: C,
    layers: Vec<Box<Layer<C,I>>>,
    // maybe transition to world unless we want separate stores for each Layer
    pub assets: warmy::Store<ggez::Context>,
}

impl<C, I> LayerStack<C, I> {
    pub fn new(world_ctx: C) -> Self {
        // may need to modify the path? 
        let opt = warmy::StoreOpt::default();
        let store = warmy::Store::new(opt).expect("Could not create store");
        LayerStack{
            world: world_ctx,
            layers: Vec::new(),
            assets: store,
        }
    }
    
    /// Pushes a new layer onto the LayerStack
    pub fn push(&mut self, layer: Box<Layer<C, I>>){
        self.layers.push(layer)
    }
    /// Pops the top Layer from the LayerStack and returns it
    /// Panics if the stack is empty
    pub fn pop(&mut self) -> Box<Layer<C, I>> {
        self.layers.pop().expect("ERROR: Attempted to pop empty stack")
    }
    /// Applies a Layer Stack Operator to the stack and returns a Layer if swapping or poping
    pub fn operate(&mut self, op: LayerStackOp<C, I>) -> Option<Box<Layer<C, I>>> {
        match op {
            LayerStackOp::None => None,
            LayerStackOp::Pop => {
                Some(self.pop())
            }
            LayerStackOp::Push(s) => {
                self.push(s);
                None
            }
            LayerStackOp::Swap(s) => {
                let old_layer = self.pop();
                self.push(s);
                Some(old_layer)
            }
        }
    }
    
    pub fn update(&mut self){
        LayerStack::update_layers(&mut self.layers, &mut self.world)
    }

    pub fn update_layers(layers: &mut [Box<Layer<C, I>>], world: &mut C){
        match layers.split_last_mut(){
            Some((layer,rest)) => {
                if layer.is_active() {
                    LayerStack::update_layers(rest,world);
                    layer.update(world);
                }
            }
            None => ()
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) {
        LayerStack::draw_layers(&mut self.layers, &mut self.world, ctx)

    } 

    pub fn draw_layers(layers: &mut [Box<Layer<C,I>>], world: &mut C, ggez_ctx: &mut ggez::Context){
        match layers.split_last_mut(){
            Some((layer,rest)) => {
                if layer.is_active() {
                    LayerStack::draw_layers(rest,world,ggez_ctx);
                    layer.draw(world,ggez_ctx).expect("Error drawing layer!")
                }
            }
            None => ()
        }
    }
}
    
/*
 * Current "build" functions with a stack of layers, some of which will function as GUI layers
 * possibly. If scenes are going to be swapped out regularly do we really want to go through the
 * trouble of moving around the GUI layers (possibly with repeated replace calls)
 * Is there a better way to implement the GUI?
 * Given that we are going to have to handle mouse clicks on pretty much every object and GUI
 * overlays this might just be the best method - Hand an event over all "active" layers with
 * relatively quick area checking. i.e. GUI only coveres X area of the scene, skip down the stack
 * for anything outside.
 */

