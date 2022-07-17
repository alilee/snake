use bevy::prelude::*;

pub type XCoord = i8;
pub type YCoord = i8;

#[derive(Component, Default, Debug)]
pub struct Location {
    pub x: XCoord,
    pub y: YCoord,
}
