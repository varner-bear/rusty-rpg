use ggez;
use ggez::graphics;
use warmy;
use resources;
use world::World;

pub struct Map {
    image: warmy::Res<resources::Image>,
}

impl Map {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let key = warmy::FSKey::new("/test_map.jpg");
        let image = world.assets.get::<_, resources::Image>(&key,ggez_ctx).unwrap(); 
        Map {
            image,
        }
    }
}


