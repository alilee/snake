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
                    .with_system(list_snakes)
                    .with_system(move_snakes)
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert(Head)
        .insert(Location::new())
        .insert(Moving::default());
}

#[derive(Component)]
struct Head;

#[derive(Component, Default, Debug)]
struct Moving {
    dir: Direction,
}

fn list_snakes(query: Query<(&Location, &Moving), With<Head>>) {
    for (loc, moving) in query.iter() {
        println!("Snake: {:?}, {:?}", loc, moving);
    }
}

fn move_snakes(mut query: Query<(&mut Location, &Moving), With<Head>>) {
    for (mut loc, moving) in query.iter_mut() {
        match loc.move_by_one(&moving.dir) {
            Ok(_) => {},
            Err(_) => {
                println!("Snake hit wall");
                //TODO handle this
            }
        }
    }
}
