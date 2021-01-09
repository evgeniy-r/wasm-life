mod random;
mod renderer;

use wasm_bindgen::prelude::*;

use crate::utils;
use fixedbitset::FixedBitSet;
use random::{JsRng, Random};
use renderer::{CanvasRenderer, Render};

const WIDTH: usize = 1600;
const HEIGHT: usize = 800;

const MIN: u8 = 2;
const MAX: u8 = 3;
const BORN: u8 = 3;

#[wasm_bindgen]
pub struct Board {
    cells: FixedBitSet,
    pub turn: u32,
    renderer: Box<dyn Render>,
    rng: Box<dyn Random>,
}

#[wasm_bindgen]
impl Board {
    pub fn for_canvas(canvas_id: &str) -> Self {
        utils::set_panic_hook();
        Self::new(
            Box::new(CanvasRenderer::new(canvas_id)),
            Box::new(JsRng::new()),
        )
    }

    fn new(renderer: Box<dyn Render>, rng: Box<dyn Random>) -> Self {
        let cells = FixedBitSet::with_capacity(WIDTH * HEIGHT);
        let turn = 0;

        Self {
            cells,
            turn,
            renderer,
            rng,
        }
    }

    pub fn fill_with_random(&mut self, area_size: usize, density: f64) {
        for y in (HEIGHT - area_size) / 2..(HEIGHT + area_size) / 2 {
            for x in (WIDTH - area_size) / 2..(WIDTH + area_size) / 2 {
                if self.rng.is_alive(density) {
                    self.cells.insert(y * WIDTH + x);
                    self.renderer.alive(x, y);
                }
            }
        }
    }

    pub fn next(&mut self) {
        let mut next_cells = self.cells.clone();

        for y in 1..HEIGHT - 1 {
            for x in 1..WIDTH - 1 {
                let index = y * WIDTH + x;

                let count = self.cells[index - WIDTH - 1] as u8
                    + self.cells[index - WIDTH] as u8
                    + self.cells[index - WIDTH + 1] as u8
                    + self.cells[index - 1] as u8
                    + self.cells[index + 1] as u8
                    + self.cells[index + WIDTH - 1] as u8
                    + self.cells[index + WIDTH] as u8
                    + self.cells[index + WIDTH + 1] as u8;

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

    fn board() -> Board {
        let renderer = MockRenderer {};
        let rng = RsRng {
            g: rand::thread_rng(),
        };
        Board::new(Box::new(renderer), Box::new(rng))
    }

    #[test]
    fn no_errors() {
        let mut board = board();

        board.fill_with_random(100, 0.3);
        board.render();
        board.next();
        board.render();

        assert_eq!(1, board.turn);
    }
}
