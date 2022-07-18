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
        .insert(Moving::default())
        // FIXME: this bundle should be in render (somehow)!
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(80.0, 80.0)),
                color: Color::rgb(0.06, 0.46, 0.06),
                ..default()
            },
            ..default()
        });
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
