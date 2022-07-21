use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

use crate::direction::Direction;
use crate::snake;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_directions);
    }
}

fn update_directions(mut query: Query<&mut snake::Moving>, mut event_keyboard: EventReader<KeyboardInput>) {
    let mut moving = query.single_mut();

    for event in event_keyboard.iter() {
        if let Some(code) = event.key_code {
            if let Ok(dir) = Direction::try_from(code) {
                if dir != moving.last_dir.get_opposite() { moving.dir = dir; }
            }
        }
    }
}
