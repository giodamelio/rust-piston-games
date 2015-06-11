use piston::event::{ RenderArgs, UpdateArgs };
use opengl_graphics::GlGraphics;

use grid::Grid;
use constants::*;

pub struct App {
    grid: Grid
}

impl App {
    pub fn new() -> App {
        App {
            grid: Grid::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::{clear};

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Render the grid
            self.grid.render(gl, &c);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
    }
}

