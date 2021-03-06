use coffee::input::keyboard::KeyCode;
use coffee::input::{self, keyboard, Input};
use serde_derive::{Deserialize, Serialize};

pub struct GameInput {
    pub keys_pressed: Vec<KeyCode>,
}

impl Input for GameInput {
    fn new() -> GameInput {
        GameInput {
            keys_pressed: Vec::new(),
        }
    }

    fn update(&mut self, event: input::Event) {
        if let input::Event::Keyboard(keyboard::Event::Input {
            key_code,
            state: input::ButtonState::Pressed,
        }) = event
        {
            self.keys_pressed.push(key_code);
        }
    }

    fn clear(&mut self) {
        self.keys_pressed.clear();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SerKey {
    Undefined,
    Right,
    Down,
    Left,
    Up,
}

impl From<KeyCode> for SerKey {
    fn from(key: KeyCode) -> SerKey {
        match key {
            KeyCode::Right => SerKey::Right,
            KeyCode::Left => SerKey::Left,
            KeyCode::Down => SerKey::Down,
            KeyCode::Up => SerKey::Up,
            _ => SerKey::Undefined,
        }
    }
}
