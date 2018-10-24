use failure::{self,Fail};
use ggez::{self,graphics};
use warmy;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self,Read};
use std::path;
//use error::*;




// This function is required as although warmy uses a VFS to manage the keys, when using 
// store::get, the stre automatically applies the prepare_key fn to each key before calling
// the load method for the given type. This means that the absolute path is ALWAYS passed to
// the load method, regardless of what your initial key is
fn warmy_to_ggez_path(path: &path::Path, root: &path::Path) -> path::PathBuf {
    let stripped_path = path.strip_prefix(root)
        .expect("warmy path is outside of the warmy store?  Should never happen.");
    path::Path::new("/").join(stripped_path)
}

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
        println!("key: {:?}, path: {:?}, key as path {:?}", key, store.root(),key.as_path());
        let path = warmy_to_ggez_path(key.as_path(), store.root());
        //graphics::Image::new(ctx, key.as_path())
        graphics::Image::new(ctx, path)
            // look up error maping later
            .map(|x| warmy::Loaded::from(Image(x)))
            //.map_err(|e| Err::from(e).compat())
    }
}


//pub struct Test(String);

//impl<C> Load <C> for Test {
    //type Key = warmy::FSKey;
    //type Error = std::io::Error;

    //fn load(
        //key: Self::Key,
        //storage: &mut Storage<C>,
        //_: &mut C
        //) -> Result<warmy::Loaded<Self>, Self::Error> {
            //let mut fh = File::open(key.as_path())?;
            //let mut s = String::new();
            //fh.read_to_string(&mut s);

            //Ok(Test(s).into())
        //}
//}

