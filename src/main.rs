use std::vec;

use wasmparser::*;
use wasmprinter::print_file;

fn main() {
    let wat = print_file("index.wasm").unwrap();
    let wat2 = wat::parse_str(&wat).unwrap();

    let mut function_names: vec::Vec<&str> = vec![];

    for payload in Parser::new(0).parse_all(&wat2) {
        match payload {
            Ok(p) => {
                match p {
                    Payload::GlobalSection(global_section) => {
                        global_section.into_iter().for_each(|g| {
                            match g {
                                Ok(global) => {
                                    match global.ty.content_type {
                                        ValType::I64 => {
                                            global.init_expr.get_operators_reader().into_iter().for_each(|op| {
                                                match op {
                                                    Ok(o) => {
                                                        match o {
                                                            Operator::I64Const { value } => {
                                                                println!("I64Const: {:?}", value);
                                                            }
                                                            _ => {}
                                                        }
                                                    }
                                                    Err(e) => {
                                                        println!("Error: {:?}", e);
                                                    }
                                                }
                                            })
                                        }
                                        _ => {}
                                    }
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        })
                    }
                    Payload::ExportSection(export_section) => {
                        export_section.into_iter().for_each(|e| {
                            match e {
                                Ok(export) => {
                                    // TODO find a way to exclude library imports such as __new, _pin ...
                                    if export.kind == ExternalKind::Func {
                                        function_names.push(export.name);
                                    }
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
                                }
                            }
                        })
                    }
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }

    println!("Function names: {:?}", function_names);
}
