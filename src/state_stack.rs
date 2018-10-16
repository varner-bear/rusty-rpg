
//! The State System for the RPG Game Engine.
//! The system uses a pushdown automata to maintain the current gamee state
//! along with any GUI's rendered over the game display.

/* Goals
 * 1) Imlement the PDA with a basic state structure
 * 2) Build a test state to display a background image
 * 3) Start a new state with a key press (stack push)
 * 4) Return to the first state with a key press (stack pop)
*/

use ggez;

/// State Trait to be implemented for our various game states in terms of a
/// common context and input type
/// Defines functions to update and render the state and handle imput events
pub trait State<C, I> {
    /// Updates the current state which could involve modifying the StateStack
    fn update(&mut self, world_ctx: &mut C) -> StateStackOp<C, I>;
    /// Renders the State using to GGEZ
    fn draw(&mut self, world_ctx: &mut C, ggex_ctx: &mut ggez::Context) -> 
        ggez::GameResult<()>;
    /// Handles an input event for an active state
    fn input(&mut self, world_ctx: &mut C, input: I, active: bool);
    /// Allows for drawing multiple states for displaying layered GUI
    /// Defaults to false until implemented
    fn draw_recursive(&self) -> bool {false} 
    fn is_active(&self) -> bool {true}

}
/// An State Stack Operation Command
/// Pushes a new state onto the stack, pops one off or replaces the current
/// state with a pup and subsequent push
pub enum StateStackOp<C, I> {
    None,
    Pop,
    Push(Box<State<C, I>>),
    Swap(Box<State<C, I>>),
}

impl<C, I> StateStackOp<C, I> {
    /// Shortcut for calling Scene Stack Operations (Swap)
    pub fn swap<S>(state: S) -> Self
    where
        S: State<C, I> + 'static,
    {
        StateStackOp::Swap(Box::new(state))   
    }

    /// Shortcut for calling Scene Stack Operations (Push)
    pub fn push<S> (state: S) -> Self
    where   
        S: State<C, I> + 'static,
    {
        StateStackOp::Push(Box::new(state))
    }
}

pub struct StateStack<C, I> {
    pub world: C,
    states: Vec<Box<State<C,I>>>,
}

impl<C, I> StateStack<C, I> {
    pub fn new(world_ctx: C) -> Self {
        StateStack{
            world: world_ctx,
            states: Vec::new(),
        }
    }
    
    /// Pushes a new state onto the StateStack
    pub fn push(&mut self, state: Box<State<C, I>>){
        self.states.push(state)
    }
    /// Pops the top State from the StateStack and returns it
    /// Panics if the stack is empty
    pub fn pop(&mut self) -> Box<State<C, I>> {
        self.states.pop().expect("ERROR: Attempted to pop empty stack")
    }
    /// Applies a State Stack Operator to the stack and returns a State if swapping or poping
    pub fn operate(&mut self, op: StateStackOp<C, I>) -> Option<Box<State<C, I>>> {
        match op {
            StateStackOp::None => None,
            StateStackOp::Pop => {
                Some(self.pop())
            }
            StateStackOp::Push(s) => {
                self.push(s);
                None
            }
            StateStackOp::Swap(s) => {
                let old_state = self.pop();
                self.push(s);
                Some(old_state)
            }
        }
    }
    
    pub fn update(&mut self){
        StateStack::update_states(&mut self.states, &mut self.world)
    }

    pub fn update_states(states: &mut [Box<State<C, I>>], world: &mut C){
        match states.split_last_mut(){
            Some((state,rest)) => {
                if state.is_active() {
                    StateStack::update_states(rest,world);
                    state.update(world);
                }
            }
            None => ()
        }
    }

    pub fn draw(&mut self, ctx: &mut ggez::Context) {
        StateStack::draw_states(&mut self.states, &mut self.world, ctx)

    } 

    pub fn draw_states(states: &mut [Box<State<C,I>>], world: &mut C, ggez_ctx: &mut ggez::Context){
        match states.split_last_mut(){
            Some((state,rest)) => {
                if state.is_active() {
                    StateStack::draw_states(rest,world,ggez_ctx);
                    state.draw(world,ggez_ctx).expect("Error drawing state!")
                }
            }
            None => ()
        }
    }
}
    
/*
 * Current "build" functions with a stack of states, some of which will function as GUI layers
 * possibly. If scenes are going to be swapped out regularly do we really want to go through the
 * trouble of moving around the GUI layers (possibly with repeated replace calls)
 * Is there a better way to implement the GUI?
 * Given that we are going to have to handle mouse clicks on pretty much every object and GUI
 * overlays this might just be the best method - Hand an event over all "active" states with
 * relatively quick area checking. i.e. GUI only coveres X area of the scene, skip down the stack
 * for anything outside.
 */

