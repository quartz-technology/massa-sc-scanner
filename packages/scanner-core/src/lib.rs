extern crate wasm_bindgen;

use std::u8;
use std::vec;
use wasmprinter::print_bytes;

use wasm_bindgen::prelude::*;
use wasmparser::*;

/// This represents a Massa smart contract scanner.
/// It is used to scan and look for exported functions and host functions.
#[wasm_bindgen]
pub struct Scanner {
    /// The WebAssembly text representation.
    wast: Vec<u8>,
}

#[wasm_bindgen]
impl Scanner {
    /// Creates a new scanner using the wasm file content.
    #[wasm_bindgen(constructor)]
    pub fn new(input: Vec<u8>) -> Result<Scanner, JsError> {
        let wat = print_bytes(input.clone()).map_err(|e| JsError::new(&e.to_string()))?;
        let wast = wat::parse_str(wat).map_err(|e| JsError::new(&e.to_string()))?;

        Ok(Scanner { wast })
    }

    /// Searches for exported function names in the smart contract.
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

    /// Searches for host function names in the smart contract.
    pub fn host_functions(&self) -> Result<vec::Vec<String>, JsError> {
        let mut host_functions: Vec<String> = Vec::new();

        for payload in Parser::new(0).parse_all(&self.wast) {
            let payload = payload.map_err(|e| JsError::new(&e.to_string()))?;

            if let Payload::ImportSection(import_section) = payload {
                for import_item in import_section {
                    let import_item = import_item.map_err(|e| JsError::new(&e.to_string()))?;

                    if import_item.module == "massa" {
                        if let TypeRef::Func(_) = import_item.ty {
                            host_functions.push(import_item.name.to_string());
                        }
                    }
                }
            }
        }

        Ok(host_functions)
    }
}
