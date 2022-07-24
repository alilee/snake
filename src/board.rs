use std::collections::HashSet;
use bevy::prelude::*;
use bevy::math::*;

use rand::prelude::*;

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
    taken: HashSet<UVec2>,
}

impl Board {
    pub fn adjacent(&self, loc: &Location, dir: &Direction) -> Option<Location> {
        let coord = loc.coord;

        let result_coord = match dir {
            Direction::Up    if coord.y < self.limit.y => uvec2(coord.x, coord.y + 1),
            Direction::Down  if coord.y > 0 => uvec2(coord.x, coord.y - 1),
            Direction::Left  if coord.x > 0 => uvec2(coord.x - 1, coord.y),
            Direction::Right if coord.x < self.limit.x => uvec2(coord.x + 1, coord.y),
            _ => return None,
        };

        Some(Location::new(result_coord))
    }

    pub fn reset(&mut self) -> Location {
        self.taken.clear();
        let coord = uvec2(self.limit.x / 2, (self.limit.y - 1) / 2 + 1);
        self.taken.insert(coord);
        Location::new(coord)
    }

    pub fn spawn_apple(&mut self) -> Location {
        let x_range = 0..=self.limit.x;
        let y_range = 0..=self.limit.y;
        let mut rng = thread_rng();
        loop {
            let coord = uvec2(rng.gen_range(x_range.clone()), rng.gen_range(y_range.clone()));
            if !self.taken.contains(&coord) {
                self.taken.insert(coord);
                return Location::new(coord);
            }
        }
    }

    pub fn block(&mut self, loc: &Location) {
        assert!(!self.taken.contains(&loc.coord));
        self.taken.insert(loc.coord);
    }

    pub fn clear(&mut self, loc: &Location) {
        self.taken.remove(&loc.coord);
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            limit: uvec2(15, 8),
            taken: HashSet::new(),
        }
    }
}
