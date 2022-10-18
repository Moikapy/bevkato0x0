use bevy::prelude::*;
mod hello_world
fn main() {
    App::new().add_system(hello_world::hello_world).run();
}

