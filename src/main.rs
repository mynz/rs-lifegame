//! Basic hello world example.

#![allow(dead_code)]

extern crate ggez;

use ggez::conf;
use ggez::event::{self, Keycode, Mod};
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;

//use std::array;

struct Grid {
    width: u32,
    height: u32,
    cells: Vec<bool>,
}

struct GridIter<'a> {
    grid: &'a Grid,
    index: usize,
}

impl<'a> std::iter::Iterator for GridIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let s = self.grid.cells.len();

        if self.index < s {
            let b = self.grid.cells[self.index];
            self.index += 1;
            Some(b)
        } else {
            None
        }
    }
}

#[test]
fn test_grid_iter() {
    assert!(true);

    let grid = Grid::new(2, 2);

    {
        let mut iter = grid.get_iter();
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == Some(false));
        assert!(iter.next() == None);
    }

    for item in grid.get_iter() {
        assert!(item == false);
    }
}

impl<'a> Grid {
    fn new(w: u32, h: u32) -> Grid {
        let size = (w * h) as usize;

        let cells = vec![false; size];

        Grid {
            width: w,
            height: h,
            cells,
        }
    }

    fn get_position_with_index(&self, index: u32) -> (u32, u32) {
        let x = index % self.width;
        let y = index / self.width;
        (x, y)
    }

    fn get_index_from_position(&self, pos: (u32, u32)) -> u32 {
        pos.1 * self.width + pos.0
    }

    fn set_cell(&mut self, pos: (u32, u32), b: bool) {
        let idx = self.get_index_from_position(pos);
        self.cells[idx as usize] = b;
    }

    fn get_iter(&self) -> GridIter {
        let ret = GridIter {
            grid: self,
            index: 0,
        };
        ret
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let span = 30;

        let mut rect = graphics::Rect {
            x: 0.0,
            y: 0.0,
            w: (span - 2) as f32,
            h: (span - 2) as f32,
        };

        let red = graphics::Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        };
        let blue = graphics::Color {
            r: 0.5,
            g: 0.5,
            b: 0.5,
            a: 1.0,
        };

        for (i, e) in self.cells.iter().enumerate() {
            let (x, y) = self.get_position_with_index(i as u32);

            let px = x * span;
            let py = y * span;

            rect.x = px as f32;
            rect.y = py as f32;

            graphics::set_color(ctx, if *e { red } else { blue });
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
        }

        Ok(())
    }
}

// First we make a structure to contain the game's state
struct MainState {
    grid: Grid,

    text: graphics::Text,
    frames: usize,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut grid = Grid::new(10, 10);
        grid.set_cell((1, 1), true);

        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;

        let s = MainState {
            grid,
            text,
            frames: 0,
        };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl ggez::event::EventHandler for MainState {
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        println!("key_down_event: {:?}, {:?}, {:?}", ctx, keycode, keymod);
        if keycode == Keycode::Q {
            ctx.quit();
        }
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.grid.draw(ctx)?;

        // Drawables are drawn from their top-left corner.
        let dest_point = graphics::Point2::new(10.0, 10.0);
        graphics::draw(ctx, &self.text, dest_point, 0.0)?;

        // draw them all.
        graphics::present(ctx);

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::get_fps(ctx));
        }

        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::conf::Conf`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("helloworld", "ggez", c).unwrap();

    // We add the CARGO_MANIFEST_DIR/resources to the filesystem's path
    // so that ggez will look in our cargo project directory for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(ctx).unwrap();
    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }
}
