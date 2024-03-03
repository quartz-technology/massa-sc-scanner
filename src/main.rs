use wasmparser::*;
use wasmprinter::print_file;

fn main() {
    let wat = print_file("index.wasm").unwrap();
    let wat2 = wat::parse_str(wat).unwrap();

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
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
