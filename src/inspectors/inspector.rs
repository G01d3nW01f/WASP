use crate::definition::Args;
use std::collections::HashMap;
use std::fs;
use wasmparser::{Parser, Payload, ExternalKind, TypeRef};

pub fn run_inspect(args: &Args) -> (Vec<Vec<u8>>, HashMap<u32, String>) {
    let file_bytes = match fs::read(&args.wat_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: Failed to read file '{}': {}", args.wat_path, e);
            std::process::exit(1);
        }
    };

    let wasm_bytes = match wat::parse_bytes(&file_bytes) {
        Ok(bytes) => bytes.into_owned(),
        Err(e) => {
            eprintln!("Error: Failed to parse WAT/Wasm format: {}", e);
            std::process::exit(1);
        }
    };

    println!("=== Wasm Structure Report ===");
    
    let parser = Parser::new(0);
    let mut function_bodies = Vec::new();
    let mut function_names = HashMap::new();
    
    let mut import_func_count = 0; // imported function count
    let mut defined_func_idx = 0;  // internal function count

    for payload in parser.parse_all(&wasm_bytes) {
        match payload {
            Ok(Payload::ImportSection(reader)) => {
                // get global name from import
                for import in reader {
                    if let Ok(imp) = import {
                        if let TypeRef::Func(_) = imp.ty {
                            // index count allocated start to 0
                            function_names.insert(import_func_count, imp.name.to_string());
                            import_func_count += 1;
                        }
                    }
                }
            }
            Ok(Payload::FunctionSection(reader)) => {
                println!("Total Functions defined: {}", reader.count());
            }
            Ok(Payload::ExportSection(reader)) => {
                for export in reader {
                    if let Ok(exp) = export {
                        if exp.kind == ExternalKind::Func {
                            function_names.insert(exp.index, exp.name.to_string());
                        }
                    }
                }
            }
            Ok(Payload::CodeSectionEntry(body)) => {
                let size = body.range().end - body.range().start;
                println!("  - Function #{}: Size {} bytes", defined_func_idx, size);
                defined_func_idx += 1;

                let start = body.range().start;
                let end = body.range().end;
                function_bodies.push(wasm_bytes[start..end].to_vec());
            }
            _ => {}
        }
    }
    println!("Imports: {}\n", import_func_count);

    (function_bodies, function_names)
}
