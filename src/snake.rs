use bevy::prelude::*;
use bevy::core::FixedTimestep;
use crate::location::*;
use crate::direction::{Direction};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(move_snakes)
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert(Head)
        .insert(Location::default())
        .insert(Moving::default());
}

#[derive(Component)]
pub(crate) struct Head;

#[derive(Component, Default)]
pub struct Moving {
    pub(crate) dir: Direction,
}

fn move_snakes(mut query: Query<(&mut Location, &Moving), With<Head>>) {
    for (mut loc, moving) in query.iter_mut() {
        *loc = loc.adjacent(&moving.dir);
    }
}
