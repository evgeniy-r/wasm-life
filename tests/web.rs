//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use life::board::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn no_errors() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    canvas.set_id("testCanvas");
    canvas.set_attribute("width", &WIDTH.to_string()).unwrap();
    canvas.set_attribute("height", &HEIGHT.to_string()).unwrap();
    document.body().unwrap().append_child(&canvas).unwrap();

    let mut board = Board::for_canvas("testCanvas", width, height);
    board.fill_with_random(100, 0.4);
    board.render();
    board.next();
    board.render();

    assert_eq!(1, board.turn);
}
