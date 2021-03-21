use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Image {}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {}
    }
}