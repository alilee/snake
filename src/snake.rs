use crate::location::*;

use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(list_snakes);
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

#[derive(Default, Debug)]
enum Direction {
    #[default]
    Up, Down, Left, Right,
}

#[derive(Component, Default, Debug)]
struct Moving {
    dir: Direction,
}

fn list_snakes(query: Query<(&Location, &Moving), With<Head>>) {
    for (loc, moving) in query.iter() {
        println!("Snake: {:?}, {:?}", loc, moving);
    }
}