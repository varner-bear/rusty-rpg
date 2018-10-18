use ggez;
use ggez::graphics;
use specs::{self,Join,World};
//mod systems;
//mod layer_stack;

//use super::layer_stack::*;
use layer_stack::*;
use layers::*;
use systems::TestSystem;


pub struct TestLayer{
    dispatcher: specs::Dispatcher<'static, 'static>,
    image: graphics::Image,
}

impl TestLayer {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let dispatcher = Self::register_systems();
        let image = graphics::Image::new(ggez_ctx,"/tile.png").unwrap();
        TestLayer {
            dispatcher,
            image,
        }   
    }
    fn register_systems() -> specs::Dispatcher<'static, 'static>{
        specs::DispatcherBuilder::new()
            .with(TestSystem, "sys_test", &[])
            .build()
    }
}

impl Layer<World, i32> for TestLayer {
    fn update(&mut self, world_ctx: &mut World) -> MyLayerStackOp {
        //self.dispatcher.dispatch(&mut world_ctx.specs_world.res);
        //println!("Updated TestLayer!");
        LayerStackOp::None
    }

    fn draw(&mut self, world_ctx: &mut World, ggez_ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::set_background_color(ggez_ctx, graphics::Color::from((64,64,64,0)));
        graphics::clear(ggez_ctx);
        graphics::draw_ex(
            ggez_ctx,
            &self.image,
            graphics::DrawParam{
                dest: graphics::Point2::new(0.0,0.0),
                ..Default::default()
            },
        )?;

        Ok(())
    }
    fn input (&mut self, world_ctx: &mut World, input: i32, active: bool) {
        ()
    }
}

