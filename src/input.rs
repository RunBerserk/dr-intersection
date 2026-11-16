use macroquad::prelude::*;

use drl_intersection::intersection::Action;

pub fn quit_pressed() -> bool {
    is_key_pressed(KeyCode::Escape)
}

pub fn reset_pressed() -> bool {
    is_key_pressed(KeyCode::R)
}

pub fn toggle_agent_pressed() -> bool {
    is_key_pressed(KeyCode::P)
}

/// Manual phase: **1** = NS green, **2** = EW green. `None` = hold current.
pub fn action() -> Option<Action> {
    let ns = is_key_down(KeyCode::Key1) || is_key_down(KeyCode::A);
    let ew = is_key_down(KeyCode::Key2) || is_key_down(KeyCode::D);
    match (ns, ew) {
        (true, false) => Some(Action::NsGreen),
        (false, true) => Some(Action::EwGreen),
        _ => None,
    }
}
