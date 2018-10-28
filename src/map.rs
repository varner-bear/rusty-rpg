use ggez;
use ggez::graphics;
use warmy;
use resources;
use world::World;



pub struct Map {
    pub image_res: warmy::Res<resources::Image>,
}

impl Map {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let key = warmy::FSKey::new("/test_map.jpg");
        let image_res = world.assets.get::<_, resources::Image>(&key,ggez_ctx).unwrap(); 
        let w = image_res.borrow().0.width();
        let h = image_res.borrow().0.height();
        let dimension = (w,h);
        Map {
            image_res,
        }
    }

    //fn get_image (self) -> ggez::graphics::Image {
        //let x = self.image_res.borrow().0;
        //x
    //}

    
}


