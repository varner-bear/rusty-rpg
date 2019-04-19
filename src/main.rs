extern crate quicksilver;

use quicksilver::prelude::*;

struct Game {
    // defines some asset for the game, which is a future of an image
    asset: Asset<Image>,
}

impl State for Game {
    // Load assets and initialize the game
    fn new() -> Result<Self> {
        // loads the image in the background while the show goes on
        let asset = Asset::new(Image::load("test_map.jpg"));
        Ok(Game{asset})
    }

    // Process keyboard + mouse, updates the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // clears the window
        window.clear(Color::WHITE)?;
        // draws the image if it is loaded, keeps going if it doesn't
        self.asset.execute(|image| {
            window.draw(&image.area().with_center((400,300)),Img(&image));
            Ok(())
        })
    }
}

fn main() {
    let settings = Settings {
        icon_path: Some("test_map.jpg"),
        ..Default::default()
    };
    run::<Game>("Quicksilver RPG", Vector::new(800,600), settings);
}
