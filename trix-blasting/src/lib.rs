use wasm_bindgen::prelude::wasm_bindgen;

mod game;

#[wasm_bindgen]
pub fn run() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    game::create_app(true).run();
}
