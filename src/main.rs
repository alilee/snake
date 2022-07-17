mod location;
mod snake;

use location::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(snake::list_snakes)
        .run();
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
    commands.spawn().insert(snake::Head).insert(Location::default()).insert(snake::Moving::default());
}
