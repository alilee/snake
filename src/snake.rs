use bevy::core::FixedTimestep;
use crate::location::*;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(list_snakes)
                    .with_system(move_snake)
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
struct Head;

#[derive(Component, Default, Debug)]
struct Moving {
    dir: Direction,
}

#[derive(Default, Debug)]
enum Direction {
    #[default]
    Up, Down, Left, Right,
}

fn list_snakes(query: Query<(&Location, &Moving), With<Head>>) {
    for (loc, moving) in query.iter() {
        println!("Snake: {:?}, {:?}", loc, moving);
    }
}

fn move_snake(mut query: Query<(&mut Location, &Moving), With<Head>>) {
    for (mut loc, moving) in query.iter_mut() {
        match moving.dir {
            Direction::Up => loc.y -= 1,
            Direction::Down => loc.y += 1,
            Direction::Left => loc.x -= 1,
            Direction::Right => loc.x += 1,
        }
    }
}
