//use specs::System;

use specs::{ReadStorage, System};
use components::Position;

pub struct TestSystem;

impl<'a> System <'a> for TestSystem {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:#?}", &position);
        }
    }


}
