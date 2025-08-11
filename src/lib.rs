pub mod game;
pub mod frontend;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<frontend::App>::new().render();
}
