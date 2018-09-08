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

struct MainState {
    text: graphics::Text,
    //canvas: graphics::Canvas,
    frames: usize,
    //draw_with_canvas: bool,
    spritebatch: graphics::spritebatch::SpriteBatch,
}

impl MainState {

    /* Creates a new main state from a given context - look into return type*/
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        /* Figure out what is going on here with the contexts */
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello World!", &font)?;
        //let canvas = graphics::Canvas::with_window_size(ctx)?;
       let image = graphics::Image::new(ctx,"/tile.png").unwrap();
       let batch = graphics::spritebatch::SpriteBatch::new(image);

        let s = MainState {
            text,
            //canvas,
            /*draw_with_canvas : false,*/
            frames: 0,
            spritebatch: batch,
        };
        Ok(s)   /* what does this do? */
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult <()> {
        let dest_point = graphics::Point2::new(10.0, 10.0);
        /*I probably don't want to use the canvas at all*/
/*
 *        if self.draw_with_canvas {
 *            println!("Drawing with canvas");
 *            graphics::set_background_color(ctx, graphics::Color::from((64,0,0,0)));
 *            graphics::clear(ctx);
 *            graphics::set_canvas(ctx, Some(&self.canvas));
 *            graphics::set_background_color(ctx, graphics::Color::from((255,255,255,128)));
 *            graphics::clear(ctx);
 *
 *            graphics::draw_ex(
 *                ctx,
 *                &self.text,
 *                graphics::DrawParam{
 *                    dest: dest_point,
 *                    color: Some(graphics::Color::from((0,0,0,255))),
 *                    ..Default::default()
 *                },
 *            )?;
 *            graphics::set_canvas(ctx, None);
 *            graphics::draw_ex(
 *                ctx,
 *                &self.canvas,
 *                graphics::DrawParam{
 *                    color: Some(graphics::Color::from((255,255,255,128))),
 *                    ..Default::default()
 *                },
 *            )?;
 *        [>} else {<]
 */
            /*println!("Drawing without canvas");*/
            graphics::set_canvas(ctx, None);
            graphics::set_background_color(ctx, graphics::Color::from((64,64,0,0)));
            graphics::clear(ctx);
            graphics::draw_ex(
                ctx,
                &self.text,
                graphics::DrawParam{
                    dest: dest_point,
                    color: Some(graphics::Color::from((0,0,0,255))),
                    ..Default::default()
                },
            )?;
         
        /*}*/

        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(200.0,300.0),
            100.0,
            0.1,
        )?;
        let time = (timer::duration_to_f64(timer::get_time_since_start(ctx)) * 1000.0) as u32;
        let cycle = 10_000;
        for x in 0..150 {
            for y in 0..150 {
                let x = x as f32;
                let y = y as f32;
                let p = graphics::DrawParam {
                    dest: graphics::Point2::new(x * 10.0, y * 10.0),
                    // scale: graphics::Point::new(0.0625, 0.0625),
                    scale: graphics::Point2::new(
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                            .cos()
                            .abs() * 0.0625,
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                            .cos()
                            .abs() * 0.0625,
                    ),
                    rotation: -2.0 * ((time % cycle) as f32 / cycle as f32 * 6.28),
                    ..Default::default()
                };
                self.spritebatch.add(p);
            }
        }
        let param = graphics::DrawParam {
            dest: graphics::Point2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).cos() * 50.0 - 350.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin() * 50.0 - 450.0,
            ),
            scale: graphics::Point2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
            ),
            rotation: ((time % cycle) as f32 / cycle as f32 * 6.28),
            offset: graphics::Point2::new(750.0, 750.0),
            ..Default::default()
        };
        graphics::draw_ex(ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();
        graphics::present(ctx); /* what does this function do*/
        
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

   let state = &mut MainState::new(ctx).unwrap();
   if let Err(e) = event::run(ctx,state){
    println!("Error encountered:{}",e);
   } else {
    println!("Game exited cleanly.");
   }
}
