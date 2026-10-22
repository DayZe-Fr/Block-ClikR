use raylib::{self, drawing::RaylibDraw};
use raylib::ffi::*;




fn main() {

    let (mut rl, thread) = raylib::init()
        .size(500, 500)
        .title("examplar")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
    	let mut draw = rl.begin_drawing(&thread);
     	draw.clear_background(Color::RAYWHITE);
     	draw.draw_text("Hello World", 100, 250, 15, Color::BLACK);
    }
}
