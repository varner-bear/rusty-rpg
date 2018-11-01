use ggez;
use ggez::graphics;
use std::f32;

use map;
//use nalgebra as na
pub struct Camera {
    pub pos: graphics::Point2,
    pub size: graphics::Rect,
    pub scale: f32,
    pub m_pos: graphics::Point2,
}

impl Camera {
    pub fn new(map: map::Map) -> Self {
        let mut pos = graphics::Point2::new(60.0,60.0);
        let mut size = graphics::Rect::new(0.0,0.0,100.0,100.0);
        size.move_to(pos);
        let mut scale = 1.0;
        let (x,y) = map.dimension;
        let mut m_pos = graphics::Point2::new(x as f32, y as f32);
        Camera {
            pos,
            size,
            scale,
            m_pos,
        }
        
    }
}

