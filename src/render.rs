use bevy::math::vec2;
use bevy::prelude::*;
use crate::location::Location;
use crate::board;
use crate::board::{Coord, XCoord, YCoord, Size, XSize, YSize, Board};
use crate::snake;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(add_head)
            .add_system(render_heads);
    }
}

const WINDOW_SIZE: Vec2 = vec2(1720.0, 960.0);

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.13, 0.73, 0.13),
            custom_size: Some(WINDOW_SIZE),
            ..default()
        },
        ..default()
    });
}

fn add_head(board: Res<Board>, mut commands: Commands, query: Query<Entity, Added<snake::Head>>) {
    let tile_size: Vec2 = WINDOW_SIZE / board.limit;
    for id in query.iter() {
        commands.entity(id).insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(tile_size),
                color: Color::rgb(0.06, 0.46, 0.06),
                ..default()
            },
            ..default()
        });
    }
}

fn render_heads(board: Res<Board>, mut query: Query<(&Location, &mut Transform), With<snake::Head>>) {
    let tile_size: Vec2 = WINDOW_SIZE / board.limit;
    for (loc, mut sprite_transform) in query.iter_mut() {
        sprite_transform.translation = get_tile_pixel_position(loc.coord, 1.0);
    }
}

fn get_tile_pixel_position(coord: UVec2, height: f32) -> Vec3 {
    Vec3::new(
        calculate_pixel_coordinate(coord.x, board::WIDTH),
        calculate_pixel_coordinate(coord.y, board::HEIGHT),
        height,
    )
}

fn calculate_pixel_coordinate(coord: Coord, size: Size) -> f32 {
    let pixel_coord = coord as f32 * TILE_SIZE;
    if size % 2 == 1 { pixel_coord } else { pixel_coord - (TILE_SIZE / 2.0) }
}
