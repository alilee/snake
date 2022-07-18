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

pub type XCoord = i8;
pub type YCoord = i8;

pub const WIDTH: XCoord = 16;
pub const HEIGHT: YCoord = 9;

fn is_in_bounds(x: XCoord, y: YCoord) -> bool {
    get_bound_range(WIDTH as i8).contains(&x) && get_bound_range(HEIGHT as i8).contains(&y)
}

fn get_bound_range(size: i8) -> RangeInclusive<i8> {
    let half = size as i8 / 2;

    if size % 2 == 1 {
        -half..=half
    } else {
        -(half - 1)..=half
    }
}
