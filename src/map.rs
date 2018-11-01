use ggez;
use ggez::graphics;
use warmy;
use resources;
use world::World;
//use nalgebra as na;
use std::f32;


pub struct Map {
    pub image_res: warmy::Res<resources::Image>,
    pub dimension : (u32,u32),
}

impl Map {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let key = warmy::FSKey::new("/test_map.jpg");
        let image_res = world.assets.get::<_, resources::Image>(&key,ggez_ctx).unwrap(); 
        let w = image_res.borrow().0.width();
        let h = image_res.borrow().0.height();
        let dimension: (u32,u32) = (w,h);
        Map {
            image_res,
            dimension,
        }
    }

    //fn get_image (self) -> ggez::graphics::Image {
        //let x = self.image_res.borrow().0;
        //x
    //}

    
}


