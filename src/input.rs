use macroquad::prelude::*;

pub fn quit_pressed() -> bool {
    is_key_pressed(KeyCode::Escape)
}
