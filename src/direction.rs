use bevy::prelude::KeyCode;

#[derive(Default, Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    #[default]
    Up, Down, Left, Right,
}

impl Direction {
    pub fn from_key_code(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Up | KeyCode::W => Some(Self::Up),
            KeyCode::Down | KeyCode::S => Some(Self::Down),
            KeyCode::Left | KeyCode::A => Some(Self::Left),
            KeyCode::Right | KeyCode::D => Some(Self::Right),
            _ => None
        }
    }

    pub fn get_opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
