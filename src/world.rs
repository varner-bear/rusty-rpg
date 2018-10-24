use ggez;
use specs;
use warmy;
use std::path;

// Custom world wrapper around Specs world and Warmy Store
pub struct World{
    pub assets: warmy::Store<ggez::Context>,
    pub specs_world: specs::World,
}

impl World{
    pub fn new(ggez_ctx: &mut ggez::Context, resource_dir: Option<path::PathBuf>) -> Self {
    let resource_pathbuf = ggez_ctx.filesystem.get_resources_dir().to_owned();
    let opt = warmy::StoreOpt::default().set_root(resource_dir.unwrap());
    let store = warmy::Store::new(opt)
        .expect("Failed to create asset store");
    println!{"Store root: {:?}",store.root()};
    let mut w = specs::World::new();
    World {
        assets: store,
        specs_world: w,
    }
    }
    
}
