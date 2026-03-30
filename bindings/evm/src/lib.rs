use wasm_bindgen::prelude::*;

mod concentrated;
mod wrappers;

#[wasm_bindgen(js_name = initPanicHook)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
