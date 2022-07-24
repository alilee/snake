use bevy::prelude::*;
use bevy::core::FixedTimestep;
use crate::board::Board;
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

fn setup(mut commands: Commands, mut board: ResMut<Board>) {
    let starting_loc = board.reset();
    commands.spawn()
        .insert(Head)
        .insert(starting_loc)
        .insert(Moving::default());
}

#[derive(Component)]
pub(crate) struct Head;

#[derive(Component, Default)]
pub struct Moving {
    pub(crate) dir: Direction,
    pub(crate) last_dir: Direction,
}

fn move_snakes(mut board: ResMut<Board>, mut query: Query<(&mut Location, &mut Moving), With<Head>>) {
    for (mut loc, mut moving) in query.iter_mut() {
        *loc = match board.adjacent(&loc, &moving.dir) {
            Some(new_loc) => {
                board.clear(&loc);
                board.block(&new_loc);
                moving.last_dir = moving.dir;
                new_loc
            },
            None => {
                println!("Snake hit a wall!");
                *moving = Moving::default();
                board.reset()
            },
        };
    }
}
