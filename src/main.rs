extern crate quicksilver;

use quicksilver::prelude::*;
struct Camera {
    rect: Rectangle,
    world_pos: Vector,
}
struct Game {
    // defines some asset for the game, which is a future of an image
    asset: Asset<Image>,
    font_test: Asset<Image>,
    camera: Camera,
}


impl State for Game {
    // Load assets and initialize the game
    fn new() -> Result<Self> {
        // loads the image in the background while the show goes on
        //let asset = Asset::new(Image::load("test_map.jpg").map(|image| image.subimage(Rectangle::new((100,100),(50,50)))));
        let asset = Asset::new(Image::load("test_map.jpg"));
        let font_mononoki = "mononoki-Regular.ttf";

        let font_test = Asset::new(Font::load(font_mononoki).and_then(|font|{
            font.render(
                "Test",&FontStyle::new(20.0, Color::BLACK),
            )
        }));
        let camera = Camera {
                        rect: Rectangle::new((5,5),(100,100)),
                        world_pos: Vector::new(5,5),
        };

        Ok(Game{asset,font_test,camera})
    }

    // Process keyboard + mouse, updates the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        /*if window.keyboard()[Key::Left].is_down() {*/
            //self.camera.world_pos = self.view.translate((-4, 0));
        //}
        //if window.keyboard()[Key::Right].is_down() {
            //self.view = self.view.translate((4, 0));
        //}
        //if window.keyboard()[Key::Down].is_down() {
            //self.view = self.view.translate((0, 4));
        //}
        //if window.keyboard()[Key::Up].is_down() {
            //self.view = self.view.translate((0, -4));
        /*}*/
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // clears the window
        window.clear(Color::WHITE)?;
        
        // draws the image if it is loaded, keeps going if it doesn't
        self.asset.execute(|image| {
            let i = image.subimage(Rectangle::new((0,0),(100,100)));
            window.draw(&i.area().with_center((400,300)),Img(&i));
            Ok(())
        })?;

        //window.draw_ex(&Rectangle::new((250,250), (100,100)), Col(Color::BLUE), Transform::rotate(0),1);
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
