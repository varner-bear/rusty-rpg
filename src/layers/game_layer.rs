
use amethyst::prelude::*;
//use amethyst::renderer::{DisplayConfig, DrawSprite, Event, Pipeline,
                         //RenderBundle, Stage, VirtualKeyCode};
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::cgmath::Vector3;
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::renderer::{
    Camera, MaterialTextureSet, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};
pub struct GameLayer;

impl<'a,'b> SimpleState<'a, 'b> for GameLayer{
   fn on_start(&mut self, data, data: StateData<GameData>) {
    
   } 
}







//use ggez;
//use ggez::graphics;

//use layers::*;
//use map::Map;
//use world::World;
//use camera::Camera;

//pub struct GameLayer{
    //map: Map,
    //camera: Camera,

//}

//impl GameLayer {
    //pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
      //let map = Map::new(ggez_ctx, world);
      //let camera = Camera::new(ggez_ctx, &map);
        //GameLayer{
            //map,
            //camera,
        //}
    //}
//}

//impl Layer<World, i32> for GameLayer {
    //fn update (&mut self, world_ctx: &mut World) -> WLayerStackOp {
        //LayerStackOp::None   
    //}

    //fn draw(&mut self, world_ctx: &mut World, ggez_ctx: &mut ggez::Context) -> 
        //ggez::GameResult<()> {
            //graphics::set_background_color(ggez_ctx, graphics::Color::from((30,30,30,0)));
            //graphics::clear(ggez_ctx);
            //self.map.draw(ggez_ctx,&self.camera);
            
           //Ok(())
        //}

    //fn input (&mut self, world_ctx: &mut World, input: i32, acctive: bool){
        //()
    //}

//}
