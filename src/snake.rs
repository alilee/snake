use crate::location::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct Head;

#[derive(Default, Debug)]
pub enum Direction {
    #[default]
    Up, Down, Left, Right,
}

#[derive(Component, Default, Debug)]
pub struct Moving {
    dir: Direction,
}

pub fn list_snakes(query: Query<(&Location, &Moving), With<Head>>) {
    for (loc, moving) in query.iter() {
        println!("Snake: {:?}, {:?}", loc, moving);
    }
}