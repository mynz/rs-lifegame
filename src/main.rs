//! Basic hello world example.

#![allow(dead_code)]

extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event::{self, Keycode, Mod, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};
//use rand::random;
use std::env;
use std::path;

struct Grid {
    width: u32,
    height: u32,
    span: u32,
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
    fn new(w: u32, h: u32, span: u32) -> Grid {
        let size = (w * h) as usize;
        let cells = vec![false; size];
        Grid {
            width: w,
            height: h,
            cells,
            span,
        }
    }

    fn get_position_with_index(&self, index: u32) -> (i32, i32) {
        let x = index % self.width;
        let y = index / self.width;
        (x as i32, y as i32)
    }

    fn get_index_from_position(&self, pos: (u32, u32)) -> usize {
        (pos.1 * self.width + pos.0) as usize
    }

    fn set_cell(&mut self, pos: (u32, u32), b: bool) {
        let idx = self.get_index_from_position(pos);
        self.cells[idx] = b;
    }

    fn toggle_cell(&mut self, pos: (u32, u32)) {
        let idx = self.get_index_from_position(pos);
        let b = !self.cells[idx];
        self.cells[idx] = b;
    }

    fn get_iter(&self) -> GridIter {
        let ret = GridIter {
            grid: self,
            index: 0,
        };
        ret
    }

    fn new_cells(&self) -> Vec<bool> {
        let size = (self.width * self.height) as usize;
        vec![false; size]
    }

    fn next_generation(&mut self) {
        let w = self.width as i32;
        let h = self.height as i32;
        let mut new_cells = self.new_cells();

        let fn_round = |v, n| {
            if v < 0 {
                n - 1
            } else if v >= n {
                0
            } else {
                v
            }
        };

        for (i, e) in self.cells.iter().enumerate() {
            let (x, y) = self.get_position_with_index(i as u32);

            let tbl = [
                (-1, 1),
                (0, 1),
                (1, 1),
                (-1, 0),
                (1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];

            let mut count = 0;
            for t in tbl.into_iter() {
                let npos = (x + t.0, y + t.1);
                let nx = fn_round(npos.0, w) as u32;
                let ny = fn_round(npos.1, h) as u32;
                let index = self.get_index_from_position((nx, ny));
                if self.cells[index] {
                    count += 1;
                }
            }

            let b = if *e {
                count == 2 || count == 3
            } else {
                count == 3
            };
            new_cells[i] = b;
        }

        self.cells = new_cells;
    }

    fn random_cells(&mut self) {
        for e in self.cells.iter_mut() {
            *e = rand::random();
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let span = self.span;

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

            let px = x * span as i32;
            let py = y * span as i32;

            rect.x = px as f32;
            rect.y = py as f32;

            graphics::set_color(ctx, if *e { red } else { blue })?;
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
    debug_mode: bool,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut grid = Grid::new(16, 20, 30);
        grid.set_cell((1, 1), true);

        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 48)?;
        let text = graphics::Text::new(ctx, "Hello world!", &font)?;

        let s = MainState {
            grid,
            text,
            frames: 0,
            debug_mode: false,
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
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, _repeat: bool) {
        println!("key_down_event: {:?}, {:?}, {:?}", ctx, keycode, keymod);

        match keycode {
            Keycode::Q => {
                ctx.quit().unwrap();
            }

            Keycode::B => {
                self.grid.toggle_cell((2, 2));
            }

            Keycode::R => {
                self.grid.random_cells();
            }

            Keycode::D => {
                self.debug_mode = !self.debug_mode;
            }

            Keycode::Space | Keycode::N => {
                self.grid.next_generation();
            }
            _ => {}
        }
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: i32,
        _y: i32,
    ) {
        println!("mouse down event occured: #{:?}", (_button, _x, _y));

        let span = self.grid.span;
        let gx = _x as u32 / span;
        let gy = _y as u32 / span;

        self.grid.toggle_cell((gx, gy));
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.grid.draw(ctx)?;


        self.frames += 1;

        // debug draw
        if self.debug_mode {
            // Drawables are drawn from their top-left corner.
            let dest_point = graphics::Point2::new(10.0, 10.0);
            graphics::draw(ctx, &self.text, dest_point, 0.0)?;
            if (self.frames % 100) == 0 {
                println!("FPS: {}", ggez::timer::get_fps(ctx));
            }
        }

        // draw them all.
        graphics::present(ctx);

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
    let mut c = conf::Conf::new();
    c.window_mode.width = 640;
    c.window_mode.height = 640;
    c.window_setup.resizable = true;
    let ctx = &mut Context::load_from_conf("rs_lifegame", "mynz", c).unwrap();

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
