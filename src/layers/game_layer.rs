use ggez;
use ggez::graphics;

use layers::*;
use map::Map;
use world::World;
use camera::Camera;

pub struct GameLayer{
    map: Map,
    camera: Camera,

}

impl GameLayer {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
      let map = Map::new(ggez_ctx, world);
      let camera = Camera::new(ggez_ctx, &map);
        GameLayer{
            map,
            camera,
        }
    }
}

impl Layer<World, i32> for GameLayer {
    fn update (&mut self, world_ctx: &mut World) -> WLayerStackOp {
        LayerStackOp::None   
    }

    fn draw(&mut self, world_ctx: &mut World, ggez_ctx: &mut ggez::Context) -> 
        ggez::GameResult<()> {
            graphics::set_background_color(ggez_ctx, graphics::Color::from((30,30,30,0)));
            graphics::clear(ggez_ctx);
            self.map.draw(ggez_ctx,&self.camera);
            
            //graphics::draw_ex(
                //ggez_ctx,
                 //the borrows the resource for rendering and unwrapes our custom image wraper
                //&self.map.image_res.borrow().0,
                //graphics::DrawParam{
                    //dest: graphics::Point2::new(1.0,1.0),
                    //scale: graphics::Point2::new(1.0,1.0),
                    //..Default::default()
                //},
            //)?;
            Ok(())
        }

    fn input (&mut self, world_ctx: &mut World, input: i32, acctive: bool){
        ()
    }

}
