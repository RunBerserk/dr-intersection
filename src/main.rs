mod input;
mod viz;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "DRL Intersection".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    viz::run().await;
}
