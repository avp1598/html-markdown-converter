use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use html2md::{parse_html_custom, StructuredPrinter, NodeData};

struct CustomDivHandler;

impl html2md::TagHandler for CustomDivHandler {
    fn handle(&mut self, tag: &html2md::Handle, printer: &mut StructuredPrinter) {
        match &tag.data {
            NodeData::Element { attrs, .. } => {
                let src = {
                    let mut src = None;
                    for attr in attrs.borrow().iter() {
                        if attr.name.local.as_ref() == "src" {
                            src = Some(attr.value.clone());
                            break;
                        }
                    }
                    src
                };
                if let Some(src) = src {
                    printer.append_str(&src);
                }
            },
            NodeData::Text { ref contents } => {
                println!("Text: {}", contents.borrow());
            },
            _ => {}

        }
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        printer.insert_newline();
    }
}

struct CustomDivHandlerFactory;

impl html2md::TagHandlerFactory for CustomDivHandlerFactory {
    fn instantiate(&self) -> Box<dyn html2md::TagHandler> {
        Box::new(CustomDivHandler)
    }
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn html_to_markdown(html: &str) -> String {
    let mut custom_handlers: HashMap<String, Box<dyn html2md::TagHandlerFactory>> = HashMap::new();
    custom_handlers.insert("div".to_owned(), Box::new(CustomDivHandlerFactory));

    let result = parse_html_custom(html, &custom_handlers);
    result
}