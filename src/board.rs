use bevy::prelude::*;
use std::ops::RangeInclusive;
use crate::App;
use crate::location::Location;
use crate::snake;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(bounds);
    }
}

fn bounds(mut query: Query<&mut Location, With<snake::Head>>) {
    for mut loc in query.iter_mut() {
        if !is_in_bounds(loc.x, loc.y) {
            println!("Snake hit a wall!");
            *loc = Location::default();
            //TODO handle this scenario
        }
    }
}

pub type Coord = i8;
pub type XCoord = Coord;
pub type YCoord = Coord;

pub type Size = u8;
pub type XSize = Size;
pub type YSize = Size;

pub const WIDTH: XSize = 16;
pub const HEIGHT: YSize = 9;

fn is_in_bounds(x: XCoord, y: YCoord) -> bool {
    get_bound_range(WIDTH).contains(&x) && get_bound_range(HEIGHT).contains(&y)
}

fn get_bound_range(size: Size) -> RangeInclusive<i8> {
    let half = (size / 2) as i8;
    if size % 2 == 1 { -half..=half } else { -(half - 1)..=half }
}
