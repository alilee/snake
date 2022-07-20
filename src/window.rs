use bevy::math::vec2;
use bevy::prelude::*;

const WIDTH: f32 = 1280.0;
const HEIGHT: f32 = 720.0;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "snake".to_string(),
            width: WIDTH,
            height: HEIGHT,
            resizable: false,
            ..default()
        });
    }
}

pub(crate) fn get_size() -> Vec2 { vec2(WIDTH, HEIGHT) }
