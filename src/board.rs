use bevy::prelude::*;
use std::ops::RangeInclusive;
use bevy::math::*;
use crate::App;
use crate::location::Location;
use crate::direction::Direction;
use crate::snake;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>();
    }
}

pub(crate) struct Board {
    pub(crate) limit: UVec2,
}

impl Board {
    pub fn adjacent(&self, loc: &Location, dir: &Direction) -> Option<Location> {
        let coord = loc.coord;
        let coord = match dir {
            Direction::Up if coord.y < self.limit.y => {
                coord + uvec2(0, 1)
            }
            Direction::Down if coord.y > 0 => {
                coord - uvec2(0, 1)
            }
            Direction::Left if coord.x > 0 => {
                coord - uvec2(1, 0)
            }
            Direction::Right if coord.x < self.limit.x => {
                coord + uvec2(1, 0)
            }
            _ => return None,
        };
        Some(Location::new(coord))
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            limit: uvec2(15, 8),
        }
    }
}