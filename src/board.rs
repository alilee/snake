use std::ops::RangeInclusive;
use crate::Vec3;

pub type XCoord = i8;
pub type YCoord = i8;

const TILE_SIZE: u8 = 80;
const BOARD_WIDTH: u8 = 16;
const BOARD_HEIGHT: u8 = 9;

pub fn get_tile_pixel_position(x: XCoord, y: YCoord, height: f32) -> Vec3 {
    Vec3::new(
        calculate_pixel_coordinate(x, BOARD_WIDTH),
        calculate_pixel_coordinate(y, BOARD_HEIGHT),
        height
    )
}

fn calculate_pixel_coordinate(coord: i8, size: u8) -> f32 {
    if size % 2 == 1 {
        coord as f32 * TILE_SIZE as f32
    } else {
        (coord as f32 - 0.5) * TILE_SIZE as f32
    }
}

pub fn is_in_bounds(x: XCoord, y: YCoord) -> bool {
    if get_bound_range(BOARD_WIDTH).contains(&x) && get_bound_range(BOARD_HEIGHT).contains(&y) {
        true
    } else {
        false
    }
}

fn get_bound_range(size: u8) -> RangeInclusive<i8> {
    let half = size as i8 / 2;

    if size % 2 == 1 {
        -half..=half
    } else {
        -(half - 1)..=half
    }
}
