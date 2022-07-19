use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

use crate::location::Location;
use crate::direction::Direction;
use crate::snake;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_directions);
    }
}

fn update_directions(mut query: Query<&mut snake::Moving, (With<Location>, With<snake::Head>)>, mut event_keyboard: EventReader<KeyboardInput>) {
    for mut moving in query.iter_mut() {
        for event in event_keyboard.iter() {
            if let Some(code) = event.key_code {
                if let Ok(dir) = Direction::try_from(code) {
                    if moving.dir.get_opposite() != dir { moving.dir = dir; }
                }
            }
        }
    }
}
