use bevy::prelude::*;
use crate::Direction::Up;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(list_snakes)
        .run();
}

#[derive(Component)]
struct Snake;

type XCoord = u8;
type YCoord = u8;

#[derive(Component, Default, Debug)]
struct Location {
    x: XCoord,
    y: YCoord,
}

#[derive(Default, Debug)]
enum Direction {
    #[default]
    Up, Down, Left, Right,
}

#[derive(Component, Default, Debug)]
struct Moving {
    dir: Direction,
}

fn list_snakes(query: Query<(&Location, &Moving), With<Snake>>) {
    for (loc, moving) in query.iter() {
        println!("Snake: {:?}, {:?}", loc, moving);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.13, 0.73, 0.13),
            custom_size: Some(Vec2::new(1280.0, 720.0)),
            ..default()
        },
        ..default()
    });
    commands.spawn().insert(Snake).insert(Location::default()).insert(Moving::default());
}
