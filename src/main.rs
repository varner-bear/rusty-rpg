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
    layers: layers::MyLayerStack,
    //draw_with_canvas: bool,
    //spritebatch: graphics::spritebatch::SpriteBatch,
}


impl MainState {
    /* Creates a new main layer from a given co:ntext - look into return type*/
    fn new(ctx: &mut Context) -> GameResult<MainState> {
    //fn new(ctx: &mut Context) -> Self{
        // creates a world and registeres the position component to it - a system can now act on
        // it - How do we get the data to use?
        let mut world = World::new();
        // Moves the world to the layer_stack
        let mut layerstack = layers::MyLayerStack::new(world);
        let initial_layer = Box::new(layers::test_layer::TestLayer::new(ctx,&mut layerstack.world));
        layerstack.push(initial_layer);
        // the MainState no longer owns the world
        //world.register::<components::Position> ();

        // Old test renderings -> moved into layers
        //let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        //let text = graphics::Text::new(ctx, "Hello World!", &font)?;
        //let image = graphics::Image::new(ctx,"/tile.png").unwrap();
        //let point = graphics::Point2::new(50.0,50.0);

        let s = MainState {
            //text,
            //canvas,
            //image, 
            //point,
            /*draw_with_canvas : false,*/
            frames: 0,
            //world,
            layers: layerstack,
            //spritebatch: batch,
        };
        Ok(s)   /* what does this do? */
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
/*    fn draw(&mut self, ctx: &mut Context) -> GameResult <()> {*/
        //let dest_point = graphics::Point2::new(10.0, 10.0);
        //graphics::set_canvas(ctx, None);

        //// sets background color
        //graphics::set_background_color(ctx, graphics::Color::from((64,64,0,0)));
        //graphics::clear(ctx);

        //// writes text to screen
        //graphics::draw_ex(
            //ctx,
            //&self.text,
            //graphics::DrawParam{
                //dest: dest_point,
                //color: Some(graphics::Color::from((0,0,0,255))),
                //..Default::default()
            //},
        //)?;
        //// draws a circle at a fixed point
        //graphics::circle(
            //ctx,
            //graphics::DrawMode::Fill,
            //graphics::Point2::new(200.0,300.0),
            //100.0,
            //0.1,
        //)?;

        //// draws the loaded image at a variable point
        //graphics::draw_ex(
            //ctx,
            //&self.image,
            //graphics::DrawParam{
                //dest: self.point,
                //..Default::default()
                
            //},
        //)?;
        //// displays FPS in console - work into display in a corner at some point
        //graphics::present(ctx); 
        //self.frames += 1;
        //if (self.frames % 100) == 0 {
            //println!("FPS: {}", ggez::timer::get_fps(ctx));
        //}

        //Ok(())
    //}
    
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

    let mut f = std::fs::File::open("conf.toml")?;
    //let conf = conf::Conf::from_toml_file(&mut f);

    let conf = match conf::Conf::from_toml_file(&mut f) {
        Ok(c) => c,
        Err(e) => conf::Conf {
    //let c = conf::Conf { /*create a new Conf - we can later load a config*/
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

    // Can't get this working? Unsure if we even want to try to read from the config
    //let mut fs = Filesystem::new("rusty-rpg","Varner")?;
    //let config = fs.read_config().unwrap();


    //f.write(b"some bytes!")?;
    //let meta = f.metadata()?;
    //println!("Permission: {:?}",meta.permissions().readonly());
    //let mut perms = meta.permissions();
    //perms.set_readonly(false);
    //std::fs::set_permissions("Conf.toml",perms)?;

    //println!("Permission: {:?}",meta.permissions().readonly());

    //let x = c.to_toml_file(&mut f).unwrap();
    // build an example toml file and go from there?
    
    // this matches correctly, need to figure out how to convert this into a usable conf
    // possibly specify our own default in another function (or just main) if it fails to load or
    // parse the file
    // i.e. let conf = match (conf from file call)
    //match conf {
        //Ok(r) => println!("Did it! {:?}",r),
        //Err(e) => println!("Nope!"),
    //}
    let ctx = &mut Context::load_from_conf("rusty-rpg","varneryo",conf).unwrap();
   
    // Add CARGO_MANIFEST_DIR/resources to the filesystem path for context building and warmy
    // conf.toml stored in the resouce directory for context building
    
    /*Adds resources folder in project dir to filesystem roots*/
    let cargo_path: Option<path::PathBuf> =
        option_env!("CARGO_MANIFEST_DIR").map(|env_path|{
           let mut res_path = path::PathBuf::from(env_path);
           res_path.push("resources");
           res_path
        });
    // unwrap the path and add it to the context builder
    if let Some(ref s) = cargo_path {
        //c_builder = c_builder.add_resource_path(s);
        ctx.filesystem.mount(s,true);
    }
        
        
        
    //if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    
    //let mut path = path::PathBuf::from(manifest_dir);
    //path.push("resources");
    //c_builder = c_builder.add_resource_path(path);
    
    // Probably want to switch over to CB so we don't have to mount the filesystem
    //let ctx = &mut c_builder.build().unwrap(); 
    /*println!("Default path: {:#?}", ctx.filesystem);*/

    let main_state = &mut MainState::new(ctx).unwrap();
    //creates a new entity in the system
    //state.world.create_entity().with(components::Position {x: 1.0, y: 7.0}).build();

    //let mut test_system = systems::TestSystem;
    //test_system.run_now(&state.world.res);

     // Actually starts the game loop 
    if let Err(e) = event::run(ctx,main_state){
    println!("Error encountered:{}",e);
    } else {
    println!("Game exited cleanly.");
    }

    Ok(())
}
