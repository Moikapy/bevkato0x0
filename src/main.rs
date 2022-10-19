use bevy::prelude::*;
mod people;
mod realm;

pub use crate::people::people::*;
pub use crate::realm::realm::hello_world;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
        .run();
}
