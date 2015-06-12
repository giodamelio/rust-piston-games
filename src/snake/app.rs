use piston::event::{ RenderArgs, UpdateArgs };
use piston::input::Button;
use opengl_graphics::GlGraphics;

use grid::Grid;
use snake::Snake;
use constants::*;

pub struct App {
    grid: Grid,
    snake: Snake
}

impl App {
    pub fn new() -> App {
        App {
            grid: Grid::new(),
            snake: Snake::new()
        }
    }

    pub fn render(&mut self, args: RenderArgs, gl: &mut GlGraphics) {
        use graphics::{clear};

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Render the grid
            self.grid.render(gl, &c);

            // Render the snake
            self.snake.render(gl, &c);
        });
    }

    pub fn update(&mut self, args: UpdateArgs) {
        self.snake.update(args);
    }

    pub fn key_press(&mut self, button: Button) {
        self.snake.key_press(button);
    }
}

