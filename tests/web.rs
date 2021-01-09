//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use life::board::*;

wasm_bindgen_test_configure!(run_in_browser);

fn prepare_canvas(id: &str, width: usize, height: usize) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_id(id);
    canvas.set_attribute("width", &width.to_string()).unwrap();
    canvas.set_attribute("height", &height.to_string()).unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();
}

#[wasm_bindgen_test]
fn fill_with_random() {
    let width = 200;
    let height = 100;

    prepare_canvas("testCanvas", width, height);

    let mut board = Board::for_canvas("testCanvas", width, height);
    board.fill_with_random(100, 0.4);
    board.render();
    board.next();
    board.render();

    assert_eq!(1, board.turn);
}

#[wasm_bindgen_test]
fn load() {
    let width = 200;
    let height = 100;

    let start_position = ".O..\n.O\n.O\n.";

    prepare_canvas("testCanvas", width, height);

    let mut board = Board::for_canvas("testCanvas", width, height);
    board.load(start_position, 100, 50);
    board.render();
    board.next();
    board.render();

    assert_eq!(1, board.turn);
}
