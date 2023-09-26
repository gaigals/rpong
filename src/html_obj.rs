use wasm_bindgen::JsCast;
use std::default::Default;

const CANVAS_CONTAINER_ID:&str = "canvas-container";
const CANVAS_ID:&str = "canvas-display";

pub struct Dom {
    window          :web_sys::Window,
    pub document    :web_sys::Document,
    pub body        :web_sys::HtmlElement,
}

impl Dom {
    pub fn new() -> Result<Dom, String> {
        let window_result = web_sys::window();
        if window_result == None {
            return Err("failed to find 'window' element".to_string());
        } 

        let window = window_result.unwrap();

        let document_result = window.document();
        if document_result == None {
            return Err("failed to find 'document' element".to_string());
        }

        let document = document_result.unwrap();

        let body_result = document.body();
        if body_result == None {
            return Err("failed to find 'body' element".to_string());
        }

        let body = body_result.unwrap(); 

        Ok(Dom{ window, document, body })
    }
}

#[derive(Debug)]
pub struct Canvas {
    pub container       :Option<web_sys::HtmlElement>,
    canvas              :Option<web_sys::HtmlCanvasElement>,
    pub context_2d      :Option<web_sys::CanvasRenderingContext2d>,
}

// Implementing the Default trait for Canvas
impl Default for Canvas {
    fn default() -> Self {
        Self {
            container: None,
            canvas: None,
            context_2d: None,
        }
    }
}

impl Canvas {
    pub fn new(dom :&Dom) -> Result<Canvas, String> {
        let mut canvas_obj = Canvas::default();

        canvas_obj.find_container(dom)?;
        canvas_obj.create_canvas();
        canvas_obj.select_canvas(dom)?;
        canvas_obj.select_context_2d()?;

        Ok(canvas_obj)
    }

    pub fn size(&mut self) -> (i32, i32) {
        let container = self.container.as_ref().unwrap();
        ( container.offset_width(), container.offset_height())
    }

    pub fn draw_pixel(&mut self, x :f64, y :f64, width :f64, height :f64) {
        self.context_2d.as_ref().unwrap().fill_rect(x, y, width, height);
    }

    fn find_container(&mut self, dom :&Dom) -> Result<(), String> {
        let container_element = dom.document.
            get_element_by_id(CANVAS_CONTAINER_ID).
            ok_or("failed to find 'canvas container' element".to_string())?;

        let container_html_element_result = container_element.
            dyn_into::<web_sys::HtmlElement>();
        if container_html_element_result.is_err() {
            return Err("failed to find 'canvas container' HTML element".to_string());
        }

        self.container = Some(container_html_element_result.unwrap());
        return Ok(())
    }

    fn create_canvas(&mut self) {
        let ( canvas_width, canvas_height) = self.size();

        let html:String = format!("<canvas id=\"{CANVAS_ID}\" width=\"{canvas_width}\" height=\"{canvas_height}\"></canvas>");

        let container = self.container.as_ref().unwrap();
        container.set_inner_html(html.as_str());
    }

    fn select_canvas(&mut self, dom :&Dom) -> Result<(), String> {
        let canvas = dom.document.get_element_by_id(CANVAS_ID).
            ok_or("failed to find 'canvas' element".to_string())?;

        // Canvas casting as HTMLElement
        let canvas_html_element = canvas.dyn_into::<web_sys::HtmlCanvasElement>();
        if canvas_html_element.is_err() {
            return Err("failed to find 'canvas' HTML element".to_string());
        }

        self.canvas = Some(canvas_html_element.unwrap());
        Ok(())
    }

    fn select_context_2d(&mut self) -> Result<(), String> {
        let canvas = self.canvas.as_ref().unwrap();

        let canvas_context_result = canvas.get_context("2d");
        if canvas_context_result.is_err() {
            return Err("failed to find 'canvas' 2d context".to_string());
        }

        let canvas_context_object_result = canvas_context_result.unwrap();
        if canvas_context_object_result == None {
            return Err("failed to obtain 'canvas' object".to_string());
        }


        let canvas_context_object = canvas_context_object_result.unwrap();

        // Canvas 2D casting as CanvasRenderingContext2d

        let canvas_context_2d_result = canvas_context_object.
            dyn_into::<web_sys::CanvasRenderingContext2d>();
        if canvas_context_2d_result.is_err() {
            return Err("failed to cast canvas object as context2D".to_string());
        }

        self.context_2d = Some(canvas_context_2d_result.unwrap());
        Ok(())
    }
}
