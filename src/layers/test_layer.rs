use ggez;
use ggez::graphics;
use specs::{self,Join,World};
use warmy;
//mod systems;
//mod layer_stack;

//use super::layer_stack::*;
use layer_stack::*;
use layers::*;
use resources;
use systems::TestSystem;
use map;

pub struct TestLayer{
    dispatcher: specs::Dispatcher<'static, 'static>,
    image: graphics::Image,
    map: graphics::Image,
    //map2: warmy::Res<resources::Image>,
}

impl TestLayer {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let dispatcher = Self::register_systems();
        let image = graphics::Image::new(ggez_ctx,"/tile.png").unwrap();
        let map = graphics::Image::new(ggez_ctx,"/test_map.jpg").unwrap();
        //let map2 = map::Map::new(ggez_ctx, world); 
         
        //let opt = warmy::StoreOpt::default();
        //let mut store = warmy::Store::new(opt).expect("Could not create store");
         //following causes a panic on loading -> Figure out what path warmy actually wants
        //let map2 = store.get::<_, resources::Image> (&warmy::FSKey::new("C:\rust_projects\rusty-rpg\resources\test_map.jpg"), ggez_ctx).unwrap();
        TestLayer {
            dispatcher,
            image,
            map,
            //map2,
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
            &self.map,
            graphics::DrawParam{
                dest: graphics::Point2::new(0.0,0.0),
                //will use to scale objects relative to map.
                //maps themselves *should* maintain native resolution butwe couuld add support
                scale: graphics::Point2::new(1.0,1.0),
                ..Default::default()
            },
        )?;
        graphics::draw_ex(
            ggez_ctx,
            &self.image,
            graphics::DrawParam{
                // Use with camera to crop things half on camera
                src: graphics::Rect::new(0.1,0.1,0.5,0.5),
                dest: graphics::Point2::new(50.0,50.0),
                ..Default::default()
            },
        )?;

        Ok(())
    }
    fn input (&mut self, world_ctx: &mut World, input: i32, active: bool) {
        ()
    }
}

