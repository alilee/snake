use bevy::prelude::*;

pub type XCoord = u8;
pub type YCoord = u8;

#[derive(Component, Default, Debug)]
pub struct Location {
    x: XCoord,
    y: YCoord,
}
