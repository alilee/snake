use bevy::prelude::*;
use bevy::math::*;
use crate::App;
use crate::location::Location;
use crate::direction::Direction;

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

        let result_coord = match dir {
            Direction::Up    if coord.y < self.limit.y => uvec2(coord.x, coord.y + 1),
            Direction::Down  if coord.y > 0            => uvec2(coord.x, coord.y - 1),
            Direction::Left  if coord.x > 0            => uvec2(coord.x - 1, coord.y),
            Direction::Right if coord.x < self.limit.x => uvec2(coord.x + 1, coord.y),
            _ => return None,
        };

        Some(Location::new(result_coord))
    }
}

impl Default for Board {
    fn default() -> Self {
        Board { limit: uvec2(15, 8) }
    }
}
