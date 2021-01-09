use js_sys::Math;

pub trait Random {
    fn is_alive(&mut self, density: f64) -> bool;
}

pub struct JsRng {}

impl Random for JsRng {
    fn is_alive(&mut self, density: f64) -> bool {
        Math::random() < density
    }
}

impl JsRng {
    pub fn new() -> Self {
        Self {}
    }
}
