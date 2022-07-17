use bevy::prelude::*;
use bevy::core::FixedTimestep;
use bevy::input::keyboard::KeyboardInput;
use bevy::sprite::Anchor;
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
            )
            .add_system(update_directions)
            .add_system(render_heads);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn()
        .insert(Head)
        .insert(Location::new())
        .insert(Moving::default())
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::TopLeft,
                custom_size: Some(Vec2::new(80.0, 80.0)),
                color: Color::rgb(0.06, 0.46, 0.06),
                ..default()
            },
            ..default()
        });
}

#[derive(Component)]
struct Head;

#[derive(Component, Default, Debug)]
struct Moving {
    last_dir: Direction,
    next_dir: Direction
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

fn render_heads(mut query: Query<(&Location, &Moving, &mut Transform), With<Head>>) {
    for (loc, _moving, mut sprite_transform) in query.iter_mut() {
        sprite_transform.translation = loc.get_translation();
    }
}
