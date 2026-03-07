pub mod game;
pub mod frontend;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    yew::Renderer::<frontend::app::App>::new().render();
}
