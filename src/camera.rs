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
    pub fn new(ggez_ctx: &mut ggez::Context, map: &map::Map) -> Self {
        
        // Gets the window dimensions from the context to build the default camera
        let (w_x, w_y) = graphics::get_size(ggez_ctx);

        // Creates a new camera size based on 80% of the window size
        let mut size = graphics::Point2::new(w_x as f32 * 0.8,w_x as f32  * 0.8);
        let mut pos = graphics::Point2::new(size.x/2.0,size.x/2.0);
        //size.move_to(pos);
        let mut scale = 1.0;
        let (x,y) = map.dimension;
        println!("Dimensions: {:?}", map.dimension);
        // need to implement actual default positioning, possibly in the center of the map
        let mut m_pos = graphics::Point2::new(x as f32/2.0, y as f32/2.0);
        //let m_pos = graphics::Point2::new(0.0,0.0);
        println!("M_pos {:?}",m_pos);
        Camera {
            pos,
            size,
            scale,
            m_pos,
        }
    }

    pub fn clip(&self, map: &map::Map) -> graphics::Rect {
        let (d_x, d_y) = map.dimension;
        let (m_x, m_y) = (self.m_pos.x,self.m_pos.y);
        println!("Map Pos X {}, Half W {}, D_X {}",m_x,self.half_width(),d_x);
        println!("Src_x {}",(m_x-self.half_width())/d_x);
        graphics::Rect::new((m_x-self.half_width())/d_x,
                            (m_y-self.half_height())/d_y,
                            (m_x+self.half_width())/d_x,
                            (m_y+self.half_height())/d_y)
    }
    pub fn width(&self) -> f32 {
        self.size.x
    }

    pub fn height(&self) -> f32 {
        self.size.y
    }

    pub fn half_width(&self) -> f32 {
        self.size.x/2.0
    }
    
    pub fn half_height(&self) -> f32 {
        self.size.y/2.0
    }

    pub fn half_size(&self) -> (f32,f32) {
        (self.half_width(), self.half_height())
    }
    pub fn draw_frame(&mut self, ggez_ctx: ggez::Context) -> ggez::GameResult<()> {
       Ok(()) 
    }
    //pub fn draw(ggez_ctx: &mut ggez::Context, map: &map::Map) -> ggez::GameResult<()> {
}

