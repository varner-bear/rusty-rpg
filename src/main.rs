extern crate quicksilver;

use quicksilver::prelude::*;

struct Game {
    // defines some asset for the game, which is a future of an image
    asset: Asset<Image>,
    font_test: Asset<Image>,
}

impl State for Game {
    // Load assets and initialize the game
    fn new() -> Result<Self> {
        // loads the image in the background while the show goes on
        let asset = Asset::new(Image::load("test_map.jpg"));
        let font_mononoki = "mononoki-Regular.ttf";

        let font_test = Asset::new(Font::load(font_mononoki).and_then(|font|{
            font.render(
                "Test",&FontStyle::new(20.0, Color::BLACK),
            )
        }));

        Ok(Game{asset,font_test})
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
        })?;

        window.draw_ex(&Rectangle::new((100,100), (32,32)), Col(Color::BLUE), Transform::rotate(45),1);
        self.font_test.execute(|image| {
            window.draw(
                &image.area().translate((2, 10)),
                Img(&image),
            );
            Ok(())
        })?;
        //println!("do we ever get here");
        Ok(())
    }
}

fn main() {
    let settings = Settings {
        icon_path: Some("test_map.jpg"),
        ..Default::default()
    };
    run::<Game>("Quicksilver RPG", Vector::new(800,600), settings);
}
