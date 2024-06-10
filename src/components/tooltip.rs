use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/assets/scripts/tooltip.js")]
extern "C" {
    pub fn tooltip(selector: &str, content: String, allow_html: bool);

    #[derive(Clone)]
    pub type Tooltip;

    #[wasm_bindgen(constructor)]
    pub fn new() -> Tooltip;

    #[wasm_bindgen(method)]
    pub fn create(this: &Tooltip, selector: &str, content: String, allow_html: bool);

    #[wasm_bindgen(method)]
    pub fn duplicate(this: &Tooltip) -> Tooltip;

    #[wasm_bindgen(method)]
    pub fn show(this: &Tooltip);

    #[wasm_bindgen(method)]
    pub fn hide(this: &Tooltip);

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Tooltip);
}

