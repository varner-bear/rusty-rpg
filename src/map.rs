use ggez;
use ggez::graphics;
use warmy;
use resources;
use world::World;
//use nalgebra as na;
use std::f32;
use camera::Camera;

pub struct Map {
    pub image_res: warmy::Res<resources::Image>,
    pub dimension : (f32,f32),
}

impl Map {
    pub fn new(ggez_ctx: &mut ggez::Context, world: &mut World) -> Self {
        let key = warmy::FSKey::new("/test_map.jpg");
        let image_res = world.assets.get::<_, resources::Image>(&key,ggez_ctx).unwrap(); 
        let w = image_res.borrow().0.width();
        let h = image_res.borrow().0.height();
        let dimension = (w as f32, h as f32);
        Map {
            image_res,
            dimension,
        }
    }

    pub fn draw(&mut self, ggez_ctx: &mut ggez::Context, c: &Camera) -> ggez::GameResult<()> {
        let image = &self.image_res.borrow().0;
        let (d_x,d_y) = self.dimension;
        let (src_x, src_y) = ((c.m_pos.x-c.size.x/2.0)/d_x,
                              (c.m_pos.y-c.size.y/2.0)/d_y);
        //println!(": {:?},{:?}",src_x,src_y);
        //println!("src: {:?},{:?}",src_x,src_y);
        //println!("src: {:?},{:?}",src_x,src_y);
        //let src = graphics::Rect::new(0.0,0.0,src_x,src_y);
        let src = graphics::Rect::new(c.m_pos.x/d_x,
                                      c.m_pos.y/d_y,
                                      c.size.x/d_x,
                                      c.size.y/d_y);
        let dest = graphics::Point2::new(c.pos.x-c.size.x/2.0,
                                         c.pos.y-c.size.y/2.0);
        let scale = graphics::Point2::new(c.scale,c.scale);
        let param = graphics::DrawParam{
            src: src,
            dest: dest,
            scale: scale, 
            ..Default::default()
        };
        graphics::draw_ex(
            ggez_ctx,
            image,
            param,
        )?;
        Ok(())
    }
    //fn get_image (self) -> ggez::graphics::Image {
        //let x = self.image_res.borrow().0;
        //x
    //}

    
}


