#![allow(non_snake_case, unused_variables, dead_code, non_camel_case_types, unused_must_use, unused_imports)]

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "One Pixel".to_owned(),
        window_height: 1,
        window_width: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);
        next_frame().await
    }
}