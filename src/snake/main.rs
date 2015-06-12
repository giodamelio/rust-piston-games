extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event::{
    Events,
    UpdateEvent,
    RenderEvent,
    PressEvent
};
// use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod app;
mod constants;
mod grid;
mod block;
mod location;
mod direction;
mod snake;

use constants::*;

fn main() {
    // Create an Glutin window.
    let window = Window::new(
        WindowSettings::new(
            "Snake",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .exit_on_esc(true)
    );

    // Create a new game and run it.
    let mut app = app::App::new();

    let mut gl = GlGraphics::new(OpenGL::_3_2);

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(r, &mut gl);
        }

        if let Some(u) = e.update_args() {
            app.update(u);
        }

        if let Some(k) = e.press_args() {
            app.key_press(k)
        }
    }
}

