use std::default::Default;

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
