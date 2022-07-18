mod location;
mod direction;
mod snake;
mod board;
mod render;
mod input;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(snake::Plugin)
        .add_plugin(render::Plugin)
        .add_plugin(input::Plugin)
        .add_plugin(board::Plugin)
        .run();
}
