use ggez;
use ggez::graphics;
use std::f32;

use map;
//use nalgebra as na
pub struct Camera {
    pub pos: graphics::Point2,
    pub size: graphics::Point2,
    pub scale: f32,
    pub m_pos: graphics::Point2,
}

impl Camera {
    pub fn new(map: &map::Map) -> Self {
        let mut pos = graphics::Point2::new(205.0,155.0);
        let mut size = graphics::Point2::new(400.0,300.0);
        //size.move_to(pos);
        let mut scale = 1.0;
        let (x,y) = map.dimension;
        println!("Dimensions: {:?}", map.dimension);
        // need to implement actual default positioning, possibly in the center of the map
        //let mut m_pos = graphics::Point2::new(x/2 as f32, y/2 as f32);
        let m_pos = graphics::Point2::new(0.0,0.0);
        println!("M_pos {:?}",m_pos);
        Camera {
            pos,
            size,
            scale,
            m_pos,
        }
    }

    pub fn draw_frame(&mut self, ggez_ctx: ggez::Context) -> ggez::GameResult<()> {
       Ok(()) 
    }
    //pub fn draw(ggez_ctx: &mut ggez::Context, map: &map::Map) -> ggez::GameResult<()> {
}

