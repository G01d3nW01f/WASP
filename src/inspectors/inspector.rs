use crate::definition::Args;
use std::collections::HashMap;
use std::fs;
use wasmparser::{Parser, Payload, ExternalKind};


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
    let mut resolved_func_idx = 0;

    for payload in parser.parse_all(&wasm_bytes) {
        match payload {
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
                println!("  - Function #{}: Size {} bytes", resolved_func_idx, size);
                resolved_func_idx += 1;

                let start = body.range().start;
                let end = body.range().end;
                function_bodies.push(wasm_bytes[start..end].to_vec());
            }
            _ => {}
        }
    }
    println!("Imports: 0\n");

    (function_bodies, function_names)
}
