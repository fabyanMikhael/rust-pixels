#![allow(non_snake_case, unused_variables, dead_code, non_camel_case_types, unused_must_use, unused_imports)]

use macroquad::prelude::*;

//this repo is made for a joke pls ignore

fn window_conf() -> Conf {
    Conf {
        window_title: "One Pixel".to_owned(),
        window_height: 27,
        window_width: 32,
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