extern crate wasm_bindgen;

use std::u8;
use std::vec;
use wasmprinter::print_bytes;

use wasm_bindgen::prelude::*;
use wasmparser::*;

#[wasm_bindgen]
pub struct Scanner {
    wast: Vec<u8>,
}

#[wasm_bindgen]
impl Scanner {
    #[wasm_bindgen(constructor)]
    pub fn new(input: Vec<u8>) -> Result<Scanner, JsError> {
        let wat = print_bytes(input.clone()).map_err(|e| JsError::new(&e.to_string()))?;
        let wast = wat::parse_str(wat).map_err(|e| JsError::new(&e.to_string()))?;

        Ok(Scanner { wast })
    }

    pub fn exported_function_names(&self) -> Result<vec::Vec<String>, JsError> {
        let mut exported_function_names: Vec<String> = Vec::new();

        // Iterate through payloads.
        for payload in Parser::new(0).parse_all(&self.wast) {
            let payload = payload.map_err(|e| JsError::new(&e.to_string()))?;

            if let Payload::ExportSection(export_section) = payload {
                // Process each export entry.
                for export in export_section {
                    let export = export.map_err(|e| JsError::new(&e.to_string()))?;
                    let first_two_chars: String = export.name.chars().take(2).collect();

                    if export.kind == ExternalKind::Func && first_two_chars != "__" {
                        exported_function_names.push(export.name.to_string());
                    }
                }
            }
        }

        Ok(exported_function_names)
    }

    pub fn host_functions(&self) -> Result<vec::Vec<String>, JsError> {
        let mut host_functions: Vec<String> = Vec::new();

        Ok(host_functions)
    }

    pub fn constants(&self) -> Result<vec::Vec<String>, JsError> {
        let mut constants: Vec<String> = Vec::new();

        Ok(constants)
    }
}
