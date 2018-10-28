extern crate ggez;
extern crate specs;
extern crate warmy;
extern crate failure;

use ggez::conf;
/*use ggez::event;*/
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::filesystem::Filesystem;
use ggez::event::{self, Axis, Button, Keycode, Mod, MouseButton, MouseState};
use ggez::timer;
/*use ggez::filesystem;*/
use std::env;
use std::path;
//use std::io::prelude;
use std::io::{self,Write};
use std::fs::File;
use specs::{Builder,Component,World,System,RunNow};


// Modules that define content
mod components;
mod systems;
mod layer_stack;
mod layers;
mod resources;
mod world;
mod map;

//use layers::test_layer;

// MainState Definition
struct MainState {
    //text: graphics::Text,
    //canvas: graphics::Canvas,
    //image: graphics::Image,
    //point: graphics::Point2,
    frames: usize,
    // Permanent Members
    // Refactor specs world into a world struct
    //world: specs::World,
    layers: layers::WLayerStack,
    //draw_with_canvas: bool,
    //spritebatch: graphics::spritebatch::SpriteBatch,
}


impl MainState {
    /* Creates a new main layer from a given co:ntext - look into return type*/
    fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> GameResult<MainState> {
    //fn new(ctx: &mut Context) -> Self{
        // creates a world and registeres the position component to it - a system can now act on
        // it - How do we get the data to use?
        let mut world = World::new();
        let mut my_world = world::World::new(ctx,resource_dir.clone());
        // Moves the world to the layer_stack
        let mut layerstack = layers::WLayerStack::new(my_world);
        
        //let initial_layer = Box::new(layers::test_layer::TestLayer::new(ctx,&mut layerstack.world));
        //layerstack.push(initial_layer);
        
        let game_layer = Box::new(layers::game_layer::GameLayer::new(ctx,&mut layerstack.world));
        layerstack.push(game_layer);
        // the MainState no longer owns the world
        //world.register::<components::Position> ();

        let s = MainState {
           frames: 0,
            layers: layerstack,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS){
            self.layers.update();
        }
        // Implement to sync everything up after an update
        // will require world module and additons
        //self.layers.world.assests.sync(ctx);
        Ok(())
    }
    fn draw(&mut self, ggez_ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ggez_ctx);
        self.layers.draw(ggez_ctx);
        graphics::present(ggez_ctx);
        Ok(())
    } 

    /* switches canvas mode on keypress */
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool){
        println!(
            "Key Pressed: {:?}, modifer {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        //self.mouse_down = true;
        println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
        //self.point = graphics::Point2::new(x as f32,y as f32);
    }
}

pub fn main() -> std::io::Result<()> {
    println!("Hello World! Starting Main!");
    // Creates a new configuration which will be used by our context
    
    let mut c_builder = ggez::ContextBuilder::new("rusty-rpg", "Varner")
        .with_conf_file(true);
    // Attempts to open the conf.toml
    let mut f = std::fs::File::open("resources/conf.toml")?;
    //let conf = conf::Conf::from_toml_file(&mut f);
    
    // Either use the loaded file or the default settings below
    let conf = match conf::Conf::from_toml_file(&mut f) {
        Ok(c) => c,
        Err(e) => conf::Conf {
        window_setup: conf::WindowSetup {
            title: "Varner's RPG Engine".to_owned(),
            icon: "".to_owned(), // Put something cool here eventually
            resizable: false, //Need to implement resize event handling
            samples: conf::NumSamples::One,
            ..Default::default()
        },
        ..Default::default()
        }
    };
    // Create a context from the given configuration
    let ctx = &mut Context::load_from_conf("rusty-rpg","varneryo",conf).unwrap();
   
    // Add CARGO_MANIFEST_DIR/resources to the filesystem path for context building and warmy
    
    /*Adds resources folder in project dir to filesystem roots*/
    let cargo_path: Option<path::PathBuf> =
        option_env!("CARGO_MANIFEST_DIR").map(|env_path|{
           let mut res_path = path::PathBuf::from(env_path);
           println!("Manifest path {:?}",res_path);
           res_path.push("resources");
           println!("Cargo path {:?}",res_path);
           res_path
        });
    // unwrap the path and add it to the context builder
    if let Some(ref s) = cargo_path {
        ctx.filesystem.mount(s,true);
    }
 
    // Create the GGEZ MainState and actually start the game loop
    let main_state = &mut MainState::new(cargo_path,ctx).unwrap();
    if let Err(e) = event::run(ctx,main_state){
    println!("Error encountered:{}",e);
    } else {
    println!("Game exited cleanly.");
    }

    Ok(())
}
