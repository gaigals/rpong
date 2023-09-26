// use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlElement};
// use web_sys::window;
use std::default::Default;
use std::fmt::format;
use std::mem;
use web_sys::console;

// let document:Element;

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
    test                :u32,
}

// Implementing the Default trait for Canvas
impl Default for Canvas {
    fn default() -> Self {
        Self {
            container: None,
            canvas: None,
            context_2d: None,
            test: 0,
        }
    }
}

impl Canvas {
    // pub fn new(dom :&Dom) -> Result<Canvas, String> {
    //     let canvas = Canvas{};
    //
    //     let container = find_container(dom)?;
    //     create_canvas(&container);
    //     let canvas = select_canvas(dom)?;
    //     let context_2d = select_context_2d(&canvas)?;
    //
    //     Ok(Canvas { container, canvas, context_2d })
    // }


    pub fn new(dom :&Dom) -> Result<Canvas, String> {
        let mut canvas_obj = Canvas::default();

        // canvas_obj.container = canvas_obj.find_container(dom)?;
        canvas_obj.find_container(dom)?;
        console::log_1(&"after".into());
        create_canvas(&canvas_obj.container.as_ref().unwrap());
        canvas_obj.canvas = Some(select_canvas(dom)?);
        // canvas_obj.context_2d = select_context_2d(canvas_obj.canvas?);
        canvas_obj.context_2d = Some(select_context_2d(canvas_obj.canvas.as_ref().unwrap())?);

        // Ok(Canvas { container, canvas, context_2d, test: 1 })
        Ok(canvas_obj)
    }

    fn find_container(&mut self, dom :&Dom) -> Result<(), String> {
        console::log_1(&"0".into());

        let container_element = dom.document.
            get_element_by_id(CANVAS_CONTAINER_ID).
            ok_or("failed to find 'canvas container' element".to_string())?;

        let container_html_element_result = container_element.
            dyn_into::<web_sys::HtmlElement>();
        if container_html_element_result.is_err() {
            return Err("failed to find 'canvas container' HTML element".to_string());
        }

        let container = container_html_element_result.unwrap();

        console::log_1(&"11".into());
        console::log_1(&format!{"{:?}\n", container}.into());

        self.container = Some(container);
        console::log_1(&format!{"{:?}\n", self}.into());
        return Ok(())
    }

}

fn find_container(dom :&Dom) -> Result<HtmlElement, String> {
    let container_element_result = dom.document.get_element_by_id(CANVAS_CONTAINER_ID);
    if container_element_result == None {
        return Err("failed to find 'canvas container' element".to_string());
    }

    let container_element = container_element_result.unwrap();

    let container_html_element_result = container_element.dyn_into::<web_sys::HtmlElement>();
    if container_html_element_result.is_err() {
        return Err("failed to find 'canvas container' HTML element".to_string());
    }

    Ok(container_html_element_result.unwrap())
}


fn create_canvas(container :&web_sys::HtmlElement) {
    let( canvas_width, canvas_height) = size(container); 

    let html:String = format!("<canvas id=\"{CANVAS_ID}\" width=\"{canvas_width}\" height=\"{canvas_height}\"></canvas>");
    container.set_inner_html(html.as_str());
}

fn select_canvas(dom :&Dom) -> Result<web_sys::HtmlCanvasElement, String> {
    let canvas_result = dom.document.get_element_by_id(CANVAS_ID);
    if canvas_result == None {
        return Err("failed to find 'canvas' element".to_string());
    }

    let canvas = canvas_result.unwrap();

    // Canvas casting as HTMLElement
    let canvas_html_element_result = canvas.dyn_into::<web_sys::HtmlCanvasElement>();
    if canvas_html_element_result.is_err() {
        return Err("failed to find 'canvas' HTML element".to_string());
    }
    
    Ok(canvas_html_element_result.unwrap())
}

fn select_context_2d(canvas :&web_sys::HtmlCanvasElement) -> Result<web_sys::CanvasRenderingContext2d, String> {
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

    let canvas_context_2d_result = canvas_context_object.dyn_into::<web_sys::CanvasRenderingContext2d>();
    if canvas_context_2d_result.is_err() {
        return Err("failed to cast canvas object as context2D".to_string());
    }

    Ok(canvas_context_2d_result.unwrap())
}

pub fn size(container :&web_sys::HtmlElement) -> (i32, i32) {
    ( container.offset_width(), container.offset_height())
}