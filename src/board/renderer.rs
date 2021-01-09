use wasm_bindgen::{Clamped, JsCast};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, ImageData};

const PIXEL_SIZE: usize = 4;

pub struct CanvasRenderer {
    data: Vec<u8>,
    width: usize,
    ctx: CanvasRenderingContext2d,
}

pub trait Render {
    fn alive(&mut self, _x: usize, _y: usize) {}

    fn empty(&mut self, _x: usize, _y: usize) {}

    fn render(&mut self) {}
}

impl Render for CanvasRenderer {
    fn alive(&mut self, x: usize, y: usize) {
        self.draw_cell(x, y, 0, 0, 0);
    }

    fn empty(&mut self, x: usize, y: usize) {
        self.draw_cell(x, y, 0xff, 0xff, 0xff);
    }

    fn render(&mut self) {
        let image_data =
            ImageData::new_with_u8_clamped_array(Clamped(&mut self.data), self.width as u32)
                .unwrap();
        self.ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}

impl CanvasRenderer {
    pub fn new(canvas_id: &str, width: usize, height: usize) -> Self {
        let data = vec![0xff; PIXEL_SIZE * width * height];

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self { data, width, ctx }
    }

    fn draw_cell(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let start = (y * self.width + x) * PIXEL_SIZE;
        self.data[start] = r;
        self.data[start + 1] = g;
        self.data[start + 2] = b;
    }
}
