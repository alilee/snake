use bevy::prelude::*;
use bevy::core::FixedTimestep;

use crate::board::Board;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(5.0))
                    .with_system(add_apple)
            );
    }
}

#[derive(Component)]
pub struct Apple;

fn add_apple(mut commands: Commands, mut board: ResMut<Board>) {
    let loc = board.spawn_apple();
    info!("spawning apple: {:?}", &loc);
    commands.spawn().insert(Apple).insert(loc);
}