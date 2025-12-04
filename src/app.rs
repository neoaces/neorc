use raylib::prelude::*;
// TODO: add imgui https://github.com/raylib-rs/raylib-rs/blob/unstable/samples/imgui.rs

pub struct SimConfig {
    width: i32,
    height: i32,
    title: &'static str,
}

pub fn start_sim() {
    let conf = SimConfig {
        width: 1920,
        height: 1080,
        title: "rc sim",
    };

    let (mut rl, thread) = raylib::init()
        .size(conf.width,conf.height) 
        .title(conf.title)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_text("neosim", 12, 12, 20, Color::WHITE);
        d.gui_button(Rectangle {x: 24.0, y: 32.0, width: 130.0, height: 20.0}, "Text");
    }
}