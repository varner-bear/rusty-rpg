use failure::{self,Fail};
use ggez::{self,graphics};
use warmy;
//use error::*;

//pub enum Err{
    //GgezError { err: ggez::GameError },
//}
pub struct Image(pub graphics::Image);

impl warmy::Load<ggez::Context> for Image {
    // Specify the warmy key type
    type Key = warmy::FSKey;
    // Specify the error type -> Research
    type Error = ggez::GameError;
    
    // Load function
    fn load(key: Self::Key,
            store: &mut warmy::Storage<ggez::Context>,
            ctx: &mut ggez::Context,
    ) -> Result<warmy::Loaded<Self>, Self::Error> {
        // for testing -> should transition to debug!
        //println!("key: {:?}, path: {:?}", key, store.root());
        graphics::Image::new(ctx, key.as_path())
            // look up error maping later
            .map(|x| warmy::Loaded::from(Image(x)))
            //.map_err(|e| Err::from(e).compat())
    }
}
