extern crate wasm_bindgen;

use std::u8;
use wasmprinter::print_bytes;

use wasm_bindgen::prelude::*;

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
pub fn scan_exported_function_names(_input: &[u8]) {
    alert("Scanning exported function names...");
}

#[wasm_bindgen]
pub fn scan_constants(_input: &[u8]) {
    alert("Scanning constants...");
}
