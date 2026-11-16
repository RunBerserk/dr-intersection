use macroquad::prelude::*;

pub async fn run() {
    loop {
        draw_frame();
        next_frame().await;
    }
}

fn draw_frame() {
    clear_background(Color::from_rgba(24, 28, 36, 255));

    let label = "Hello, Intersection!";
    let font_size = 40.0;
    let text_dims = measure_text(label, None, font_size as u16, 1.0);
    draw_text(
        label,
        (screen_width() - text_dims.width) * 0.5,
        (screen_height() + text_dims.height) * 0.5,
        font_size,
        WHITE,
    );

    let hint = "Esc to quit";
    let hint_size = 18.0;
    let hint_dims = measure_text(hint, None, hint_size as u16, 1.0);
    draw_text(
        hint,
        (screen_width() - hint_dims.width) * 0.5,
        screen_height() - 32.0,
        hint_size,
        GRAY,
    );
}
