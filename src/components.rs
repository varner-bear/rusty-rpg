use specs::*;

// Components
/////////////

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
