use bevy::prelude::KeyCode;

#[derive(Default, PartialEq, Copy, Clone)]
pub enum Direction {
    #[default]
    Up, Down, Left, Right,
}

impl Direction {
    pub fn get_opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl TryFrom<KeyCode> for Direction {
    type Error = ();

    fn try_from(key_code: KeyCode) -> Result<Self, Self::Error> {
        match key_code {
            KeyCode::Up | KeyCode::W => Ok(Self::Up),
            KeyCode::Down | KeyCode::S => Ok(Self::Down),
            KeyCode::Left | KeyCode::A => Ok(Self::Left),
            KeyCode::Right | KeyCode::D => Ok(Self::Right),
            _ => Err(())
        }
    }
}
