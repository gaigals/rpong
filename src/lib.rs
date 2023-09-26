mod html_obj;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::CanvasRenderingContext2d;
use html_obj::*;
use web_sys::console;
// use log::*;


#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console::log_1(&"Hello from rust".into());

    let dom = html_obj::Dom::new()?;
    console::log_1(&"Here".into());
    let canvas = html_obj::Canvas::new(&dom)?;


    // canvas.create_canvas();
    // log::console_log!("Hello from rust macro");
    

    // let window:web_sys::Window = web_sys::window().expect("no global `window` exists");
    // let document:web_sys::Document = window.document().expect("should have a document on window");
    // let body:web_sys::HtmlElement = document.body().expect("document should have a body");

    // let canvas_container:web_sys::Element = dom.document.get_element_by_id(CANVAS_CONTAINER_ID).unwrap();
    // let container_html_element:web_sys::HtmlElement = canvas_container
    //     .dyn_into::<web_sys::HtmlElement>()
    //     .unwrap();



    // let html:String = format!("<canvas id=\"{CANVAS_ID}\" width=\"{canvas_width}\" height=\"{canvas_height}\"></canvas>");
    // container_html_element.set_inner_html(html.as_str());

    // let canvas:web_sys::Element = document.get_element_by_id(CANVAS_ID).unwrap();
    // let canvas_html_element: web_sys::HtmlCanvasElement = canvas
    //                     .dyn_into::<web_sys::HtmlCanvasElement>()
    //                     .map_err(|_| ())
    //                     .unwrap();

    // let canvas_context:CanvasRenderingContext2d = canvas_html_element.get_context("2d")
    //                     .unwrap()
    //                     .unwrap()
    //                     .dyn_into::<web_sys::CanvasRenderingContext2d>()
    //                     .unwrap();

    // Required to adjust new size of canvas offset.
    canvas.context_2d.scale(1., 1.).unwrap();

    let( canvas_width, canvas_height) = size(&canvas.container); 
    
    // Dots left.
    canvas.context_2d.fill_rect(0., 0., 2., 2.);
    canvas.context_2d.fill_rect(0., canvas_height as f64- 2., 2., 2.);
    // Dots right;
    canvas.context_2d.fill_rect(canvas_width as f64 - 2., 0., 2., 2.);
    canvas.context_2d.fill_rect(canvas_width as f64, canvas_height as f64- 2., 2., 2.);
    

    // let val = dom.document.create_element("p")?;
    // val.set_inner_html(format!("width:{}, height:{}", 
    //     canvas_width,
    //     canvas_height,
    // ).as_str());

    // dom.body.append_child(&val)?;

    Ok(())
}
