extern crate quicksilver;

use quicksilver::prelude::*;

struct Game;

impl State for Game {
    // Load assets and initialize the game
    fn new() -> Result<Self> {
        Ok(Self)
    }

    // Process keyboard + mouse, updates the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }
}

fn main() {
    let settings = Settings {
        ..Default::default()
    };
    run::<Game>("Quicksilver RPG", Vector::new(800,600), settings);
}
