use bevy::prelude::*;
use crate::location::Location;
use crate::board;
use crate::board::{XCoord, YCoord};
use crate::snake;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_heads)
            .add_startup_system(setup);
    }
}

fn render_heads(mut query: Query<(&Location, &mut Transform), With<snake::Head>>) {
    for (loc, mut sprite_transform) in query.iter_mut() {
        sprite_transform.translation = get_tile_pixel_position(loc.x, loc.y, 1.0);
    }
}

const TILE_SIZE: f32 = 80.0;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.13, 0.73, 0.13),
            custom_size: Some(Vec2::new(TILE_SIZE * board::WIDTH as f32, TILE_SIZE * board::HEIGHT as f32)),
            ..default()
        },
        ..default()
    });
}

fn get_tile_pixel_position(x: XCoord, y: YCoord, height: f32) -> Vec3 {
    Vec3::new(
        calculate_pixel_coordinate(x, board::WIDTH),
        calculate_pixel_coordinate(y, board::HEIGHT),
        height,
    )
}

fn calculate_pixel_coordinate(coord: i8, size: i8) -> f32 {
    let coord = coord as f32 * TILE_SIZE;
    if size % 2 == 1 {
        coord
    } else {
        coord - (TILE_SIZE / 2.0)
    }
}


