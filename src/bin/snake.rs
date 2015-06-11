extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::fmt;

use piston::window::WindowSettings;
use piston::event::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics::{ Context };

const BLOCK_SIZE: u32 = 5;

const WINDOW_WIDTH: u32 = 500;
const WINDOW_HEIGHT: u32 = 500;

const WIDTH_IN_BLOCKS: u32 = WINDOW_WIDTH / BLOCK_SIZE;
const HEIGHT_IN_BLOCKS: u32 = WINDOW_HEIGHT / BLOCK_SIZE;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.1];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.1];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.1];

// The Direction a block is facing
#[derive(PartialEq, Debug)]
enum Direction {
	Up,
	Down,
	Left,
	Right
}

struct App {
    grid: Grid
}

impl App {
    fn new() -> App {
        App {
            grid: Grid::new()
        }
    }
}

struct Grid {
    grid: Vec<Vec<Block>>
}

#[derive(Debug)]
struct Block {
    location: Location,
    blockType: BlockType
}

#[derive(Debug)]
enum BlockType {
    Wall,
    Empty,
    SnakeHead,
    SnakeBody,
    Apple
}

#[derive(Debug)]
struct Location {
    x: u32,
    y: u32
}

impl App {
    fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::{clear};

        gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Render the grid
            self.grid.render(gl, &c);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
    }
}

impl Grid {
    fn new() -> Grid {
        // Fill in the walls of the grid
        let mut rows: Vec<Vec<Block>> = vec!();
        rows.reserve((WINDOW_HEIGHT / BLOCK_SIZE) as usize);
        for rownum in 0..HEIGHT_IN_BLOCKS {
            let mut row: Vec<Block> = vec!();
            for colnum in 0..WIDTH_IN_BLOCKS {
                let wallType = match (colnum, rownum) {
                    (x, y) if x == 0 => BlockType::Wall,
                    (x, y) if x == WIDTH_IN_BLOCKS - 1 => BlockType::Wall,
                    (x, y) if y == 0 => BlockType::Wall,
                    (x, y) if y == HEIGHT_IN_BLOCKS - 1 => BlockType::Wall,
                    _ => BlockType::Empty
                };

                row.push(Block {
                    blockType: wallType,
                    location: Location {
                        x: colnum * BLOCK_SIZE,
                        y: rownum * BLOCK_SIZE
                    }
                });
            }
            rows.push(row);
        }

        let mut grid = Grid {
            grid: rows
        };
        grid
    }

    fn render(&self, gl: &mut GlGraphics, c: &Context) {
        use graphics::*;

        for row in self.grid.iter() {
            for block in row.iter() {
                let square = rectangle::square(
                    block.location.x as f64,
                    block.location.y as f64,
                    BLOCK_SIZE as f64
                );

                match block.blockType {
                    BlockType::Wall =>
                        rectangle(WHITE, square, c.transform, gl),
                    BlockType::Empty =>
                        rectangle(BLACK, square, c.transform, gl),
                    _ =>
                        rectangle(RED, square, c.transform, gl),
                }
            }
        }
    }
}

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
    let mut app = App::new();

    let mut gl = GlGraphics::new(OpenGL::_3_2);

    for e in window.events() {
        if let Some(r) = e.render_args() {
            app.render(&r, &mut gl);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

