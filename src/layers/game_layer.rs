use ggez;
use ggez::graphics;

use layers::*;
use map::Map;
use world::World;

pub struct GameLayer{
    map: Map
}

impl GameLayer {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
      let map = Map::new(ggez_ctx, world); 
        GameLayer{
            map,
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
            Ok(())
        }

    fn input (&mut self, world_ctx: &mut World, input: i32, acctive: bool){
        ()
    }

}
