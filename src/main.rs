extern crate amethyst;

use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
pub struct GameLayer;

impl<'a,'b> SimpleState<'a, 'b> for GameLayer{
    
}

fn main() -> amethyst::Result<()> {
    
    // start the amethyst logger with default configuration
    amethyst::start_logger(Default::default());

    let config_path = "./resources/display_config.ron";
    let config = DisplayConfig::load(&config_path);
    
    // some default rendering code
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
        // defines the background color
        .clear_target([0.0,0.0,0.0,1.0], 1.0)
        .with_pass(DrawSprite::new()),
    );
    // see pong example for "with_basic_renderer
    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;
    //Application is the root obect for amethyst     
    let mut game = Application::new("./", GameLayer, game_data)?;
    // Begins the game loop
    // Will continue until the State returns Trans::Quit or state stack is empty
    game.run();
    Ok(())
}
