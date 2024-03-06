extern crate wasm_bindgen;

use std::vec;
use std::u8;
use wasmprinter::print_bytes;

use wasm_bindgen::prelude::*;
use wasmparser::*;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, World!");
}

#[wasm_bindgen]
pub fn scan(input: &[u8]) {
    // let wat = print_file("index.wasm").unwrap();
    // let wat2 = wat::parse_str(&wat).unwrap();

    let wat = print_bytes(input).unwrap();
    let _wat2 = wat::parse_str(&wat).unwrap();
}

#[wasm_bindgen]
pub fn scan_host_functions(_input: &[u8]) {
    alert("Scanning host functions...");
}

#[wasm_bindgen]
pub fn scan_exported_function_names(_input: &[u8]) -> Result<vec::Vec<String>, JsValue> {
    let mut function_names: vec::Vec<String> = vec![];

    let wat = print_bytes(_input).unwrap();
    let wat2 = wat::parse_str(&wat).unwrap();

    for payload in Parser::new(0).parse_all(&wat2) {
        match payload {
            Ok(p) => {
                match p {
                    Payload::ExportSection(export_section) => {
                        for result in export_section {
                            match result {
                                Ok(export) => {
                                    let first_two_chars: String = export.name.chars().take(2).collect();
                                    if export.kind == ExternalKind::Func && first_two_chars != "__" {
                                        function_names.push(export.name.to_string());
                                    }
                                }
                                Err(e) => return Err(JsValue::from_str(&e.to_string())),
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => return Err(JsValue::from_str(&e.to_string())),
        }
    }
    Ok(function_names)
}

#[wasm_bindgen]
pub fn scan_constants(_input: &[u8]) {
    alert("Scanning constants...");
}
