mod web;

use wasm_bindgen::prelude::*;
use web_sys::console;
use web::dom::*;
use web::canvas::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Hello from rust".into());

    let dom = Dom::new()?;
    let mut canvas = Canvas::new(&dom)?;

    // Required to adjust new size of canvas offset.
    canvas.context_2d.as_ref().unwrap().scale(1., 1.).unwrap();

    let( canvas_width, canvas_height) = canvas.size();
    
    // Dots left.
    canvas.draw_pixel(0., 0., 2., 2.);
    canvas.draw_pixel(0., canvas_height as f64- 2., 2., 2.);
    // Dots right;
    canvas.draw_pixel(canvas_width as f64 - 2., 0., 2., 2.);
    canvas.draw_pixel(canvas_width as f64 - 2., canvas_height as f64 - 2., 2., 2.);

    Ok(())
}
