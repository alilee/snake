use bevy::prelude::*;
use bevy::sprite::Anchor;
use crate::location::Location;
use crate::board::Board;
use crate::{snake, window};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(add_heads)
            .add_system(move_heads);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.13, 0.73, 0.13),
            custom_size: Some(window::get_size()),
            ..default()
        },
        ..default()
    });
}

fn add_heads(board: Res<Board>, mut commands: Commands, query: Query<Entity, Added<snake::Head>>) {
    for id in query.iter() {
        commands.entity(id).insert_bundle(SpriteBundle {
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                custom_size: Some(get_tile_size(board.limit)),
                color: Color::rgb(0.06, 0.46, 0.06),
                ..default()
            },
            ..default()
        });
    }
}

fn move_heads(board: Res<Board>, mut query: Query<(&Location, &mut Transform), With<snake::Head>>) {
    for (loc, mut sprite_transform) in query.iter_mut() {
        sprite_transform.translation = get_tile_pixel_position(loc.coord, 1.0, &*board);
    }
}

fn get_tile_size(board_limit: UVec2) -> Vec2 {
    window::get_size() / (board_limit + 1).as_vec2()
}

fn get_tile_pixel_position(coord: UVec2, height: f32, board: &Board) -> Vec3 {
    let tile_size = get_tile_size(board.limit);

    Vec3::new(
        calculate_pixel_coordinate(coord.x, tile_size.x, window::get_size().x),
        calculate_pixel_coordinate(coord.y, tile_size.y, window::get_size().y),
        height,
    )
}

fn calculate_pixel_coordinate(coord: u32, tile_size: f32, window_size: f32) -> f32 {
    -(window_size / 2.0) + (coord as f32 * tile_size)
}
