mod viz;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "DRL Intersection — Hello World".to_owned(),
        window_width: 800,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    viz::run().await;
}
