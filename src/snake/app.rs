use piston::event::{ RenderArgs, UpdateArgs };
use piston::input::Button;
use opengl_graphics::GlGraphics;

use grid::Grid;
use snake::Snake;
use constants::*;

pub struct App {
    grid: Grid,
    snake: Snake,
    paused: bool
}

impl App {
    pub fn new() -> App {
        App {
            grid: Grid::new(),
            snake: Snake::new(),
            paused: true
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
        if self.paused { return }
        self.snake.update(args);
    }

    pub fn key_press(&mut self, button: Button) {
        use piston::input::Button::Keyboard;
        use piston::input::keyboard::Key;

        // Unpause on any key
        if self.paused {
            self.paused = false;
        }

        match button {
            // Pause on p
            Keyboard(Key::P) => self.paused = true,
            _ => {},
        }

        // Forward the keypress to the snake
        self.snake.key_press(button);
    }
}

