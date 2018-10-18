extern crate ggez;
extern crate specs;

use ggez::conf;
/*use ggez::event;*/
use ggez::graphics;
use ggez::{Context, GameResult,};
use ggez::event::{self, Axis, Button, Keycode, Mod, MouseButton, MouseState};
use ggez::timer;
/*use ggez::filesystem;*/
use std::env;
use std::path;

use specs::{Builder,Component,World,System,RunNow};


// Modules that define content
mod components;
mod systems;
mod layer_stack;
mod test_layer;

// MainState Definition
struct MainState {
    text: graphics::Text,
    //canvas: graphics::Canvas,
    image: graphics::Image,
    point: graphics::Point2,
    frames: usize,
    // Permanent Members
    // Refactor specs world into a world struct
    //world: specs::World,
    layers: test_layer::MyLayerStack,
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
        let mut layerstack = test_layer::MyLayerStack::new(world);
        let initial_layer = Box::new(test_layer::TestLayer::new(ctx,&mut layerstack.world));
        layerstack.push(initial_layer);
        // the MainState no longer owns the world
        //world.register::<components::Position> ();
        /* Figure out what is going on here with the contexts */
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello World!", &font)?;
        //let canvas = graphics::Canvas::with_window_size(ctx)?;
        let image = graphics::Image::new(ctx,"/tile.png").unwrap();
        let point = graphics::Point2::new(50.0,50.0);
       //let batch = graphics::spritebatch::SpriteBatch::new(image);

        let s = MainState {
            text,
            //canvas,
            image, 
            point,
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

    fn draw(&mut self, ctx: &mut Context) -> GameResult <()> {
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::set_canvas(ctx, None);

        // sets background color
        graphics::set_background_color(ctx, graphics::Color::from((64,64,0,0)));
        graphics::clear(ctx);

        // writes text to screen
        graphics::draw_ex(
            ctx,
            &self.text,
            graphics::DrawParam{
                dest: dest_point,
                color: Some(graphics::Color::from((0,0,0,255))),
                ..Default::default()
            },
        )?;
        // draws a circle at a fixed point
        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(200.0,300.0),
            100.0,
            0.1,
        )?;

        // draws the loaded image at a variable point
        graphics::draw_ex(
            ctx,
            &self.image,
            graphics::DrawParam{
                dest: self.point,
                ..Default::default()
                
            },
        )?;
        // displays FPS in console - work into display in a corner at some point
        graphics::present(ctx); 
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

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
        self.point = graphics::Point2::new(x as f32,y as f32);
    }
}

pub fn main() {
    println!("Hello World! Starting Main!");
    let c = conf::Conf { /*create a new Conf - we can later load a config*/
        window_setup: conf::WindowSetup {
            samples: conf::NumSamples::Two,
            ..Default::default()
        },
        ..Default::default()
    };
    /*create a new context*/
    let ctx = &mut Context::load_from_conf("rusty-rpg","varneryo",c).unwrap();
    /*println!("Default path: {:#?}", ctx.filesystem);*/
    /*Adds resources folder in project dir to filesystem roots*/
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    let mut path = path::PathBuf::from(manifest_dir);
    path.push("resources");
    ctx.filesystem.mount(&path,true);
    /*println!("Default path: {:#?}", ctx.filesystem);*/
    }

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
}
