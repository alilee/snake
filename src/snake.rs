use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::input::keyboard::KeyboardInput;
use crate::location::*;
use crate::direction::{Direction};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1.0))
                    .with_system(list_snakes.before(move_snakes))
                    .with_system(move_snakes)
            )
            .add_system(update_directions);
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
    last_dir: Direction,
    next_dir: Direction
}

fn list_snakes(query: Query<(&Location, &Moving), With<Head>>) {
    for (loc, moving) in query.iter() {
        println!("Snake: {:?}, {:?}", loc, moving);
    }
}

fn move_snakes(mut query: Query<(&mut Location, &mut Moving), With<Head>>) {
    for (mut loc, mut moving) in query.iter_mut() {
        moving.last_dir = moving.next_dir;
        match loc.move_by_one(&moving.next_dir) {
            Ok(_) => {},
            Err(_) => {
                println!("Snake hit wall");
                //TODO handle this
            }
        }
    }
}

fn update_directions(mut query: Query<&mut Moving, (With<Location>, With<Head>)>, mut event_keyboard: EventReader<KeyboardInput>) {
    for mut moving in query.iter_mut() {
        for event in event_keyboard.iter() {
            if let Some(code) = event.key_code {
                if let Some(dir) = Direction::from_key_code(code) {
                    if moving.last_dir.get_opposite() != dir { moving.next_dir = dir; }
                }
            }
        }
    }
}
