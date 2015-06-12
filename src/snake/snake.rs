use graphics::{ Context };
use opengl_graphics::GlGraphics;
use piston::event::UpdateArgs;
use piston::input::Button;

use location::Location;
use direction::Direction;
use constants::*;

pub struct Snake {
    pub location: Location,
    pub direction: Direction
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            location: Location {
                x: 250,
                y: 250
            },
            direction: Direction::Up
        }
    }

    pub fn render(&self, gl: &mut GlGraphics, c: &Context) {
        use graphics::*;

        let square = rectangle::square(
            self.location.x as f64,
            self.location.y as f64,
            BLOCK_SIZE as f64
        );
        rectangle(RED, square, c.transform, gl);
    }

    pub fn update(&mut self, args: UpdateArgs) {
        match self.direction {
            Direction::Up => self.location.y -= 1,
            Direction::Down => self.location.y += 1,
            Direction::Right => self.location.x += 1,
            Direction::Left => self.location.x -= 1
        }
    }

    pub fn key_press(&mut self, button: Button) {
        use piston::input::Button::Keyboard;
        use piston::input::keyboard::Key;

        match button {
            Keyboard(Key::Up) => self.direction = Direction::Up,
            Keyboard(Key::Down) => self.direction = Direction::Down,
            Keyboard(Key::Right) => self.direction = Direction::Right,
            Keyboard(Key::Left) => self.direction = Direction::Left,
            _ => println!("Unhandled key: {:?}", button),
        };
    }
}
