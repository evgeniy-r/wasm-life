mod random;
mod renderer;

use wasm_bindgen::prelude::*;

use crate::utils;
use fixedbitset::FixedBitSet;
use random::{JsRng, Random};
use renderer::{CanvasRenderer, Render};

const ALIVE_CHAR: char = 'O';

const MIN: u8 = 2;
const MAX: u8 = 3;
const BORN: u8 = 3;

#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    cells: FixedBitSet,
    pub turn: u32,
    renderer: Box<dyn Render>,
    rng: Box<dyn Random>,
}

#[wasm_bindgen]
impl Board {
    pub fn for_canvas(canvas_id: &str, width: usize, height: usize) -> Self {
        utils::set_panic_hook();
        Self::new(
            Box::new(CanvasRenderer::new(canvas_id, width, height)),
            Box::new(JsRng::new()),
            width,
            height,
        )
    }

    fn new(renderer: Box<dyn Render>, rng: Box<dyn Random>, width: usize, height: usize) -> Self {
        let cells = FixedBitSet::with_capacity(width * height);
        let turn = 0;

        Self {
            width,
            height,
            cells,
            turn,
            renderer,
            rng,
        }
    }

    pub fn fill_with_random(&mut self, area_size: usize, density: f64) {
        let width = self.width;
        for y in (self.height - area_size) / 2..(self.height + area_size) / 2 {
            for x in (width - area_size) / 2..(width + area_size) / 2 {
                if self.rng.is_alive(density) {
                    self.cells.insert(y * width + x);
                    self.renderer.alive(x, y);
                }
            }
        }
    }

    pub fn load(&mut self, s: &str, start_x: usize, start_y: usize) {
        let width = self.width;

        for (y, line) in s.split("\n").enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == ALIVE_CHAR {
                    self.cells.insert((start_y + y) * width + start_x + x);
                    self.renderer.alive(start_x + x, start_y + y);
                }
            }
        }
    }

    pub fn next(&mut self) {
        let width = self.width;
        let mut next_cells = self.cells.clone();

        for y in 1..self.height - 1 {
            for x in 1..width - 1 {
                let index = y * width + x;

                let count = self.cells[index - width - 1] as u8
                    + self.cells[index - width] as u8
                    + self.cells[index - width + 1] as u8
                    + self.cells[index - 1] as u8
                    + self.cells[index + 1] as u8
                    + self.cells[index + width - 1] as u8
                    + self.cells[index + width] as u8
                    + self.cells[index + width + 1] as u8;

                if self.cells[index] {
                    if count < MIN || count > MAX {
                        next_cells.toggle(index);
                        self.renderer.empty(x, y);
                    }
                } else {
                    if count == BORN {
                        next_cells.toggle(index);
                        self.renderer.alive(x, y);
                    }
                }
            }
        }

        self.cells = next_cells;
        self.turn += 1;
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests {
    use super::*;
    use rand::rngs::ThreadRng;
    use rand::Rng;

    struct MockRenderer {}

    impl Render for MockRenderer {}

    struct RsRng {
        g: ThreadRng,
    }

    impl Random for RsRng {
        fn is_alive(&mut self, density: f64) -> bool {
            self.g.gen_bool(density)
        }
    }

    fn board(width: usize, height: usize) -> Board {
        let renderer = MockRenderer {};
        let rng = RsRng {
            g: rand::thread_rng(),
        };
        Board::new(Box::new(renderer), Box::new(rng), width, height)
    }

    #[test]
    fn fill_with_random() {
        let mut board = board(200, 200);

        board.fill_with_random(100, 0.3);
        board.render();
        board.next();
        board.render();

        assert_eq!(1, board.turn);
    }

    #[test]
    fn load() {
        let start_position = ".O..\n.O\n.O\n.";
        let mut board = board(6, 6);

        board.load(start_position, 1, 1);
        board.render();

        assert!(!board.cells[6 + 1]);
        assert!(board.cells[6 + 2]);
        assert!(!board.cells[6 + 3]);
        assert!(!board.cells[2 * 6 + 1]);
        assert!(board.cells[2 * 6 + 2]);
        assert!(!board.cells[2 * 6 + 3]);
        assert!(!board.cells[3 * 6 + 1]);
        assert!(board.cells[3 * 6 + 2]);
        assert!(!board.cells[3 * 6 + 3]);

        board.next();
        board.render();

        assert_eq!(1, board.turn);

        assert!(!board.cells[6 + 1]);
        assert!(!board.cells[6 + 2]);
        assert!(!board.cells[6 + 3]);
        assert!(board.cells[2 * 6 + 1]);
        assert!(board.cells[2 * 6 + 2]);
        assert!(board.cells[2 * 6 + 3]);
        assert!(!board.cells[3 * 6 + 1]);
        assert!(!board.cells[3 * 6 + 2]);
        assert!(!board.cells[3 * 6 + 3]);
    }
}
