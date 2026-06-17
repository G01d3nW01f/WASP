use crate::definition::{Args, OutputFormat};
use std::collections::HashMap;
use wasmparser::{Operator, WasmFeatures};

pub fn run_decompile(args: &Args, function_bodies: Vec<Vec<u8>>, function_names: HashMap<u32, String>) {
    println!("--- Running Decompiler ---");

    if !args.no_dfa {
        println!("[DFA] Analyzing control flow graph and expression stack...\n");
    }

    match args.format {
        OutputFormat::Text => {
            println!("=== Decompiled Pseudo-Code (Text) ===");
            
            for (idx, body_bytes) in function_bodies.iter().enumerate() {
                let u32_idx = idx as u32;
                
                
                let func_name = function_names
                    .get(&u32_idx)
                    .cloned()
                    .unwrap_or_else(|| format!("function_{}", idx));

                let binary_reader = wasmparser::BinaryReader::new(body_bytes, 0, WasmFeatures::default());
                let body = wasmparser::FunctionBody::new(binary_reader);
                let op_reader = body.get_operators_reader().unwrap();

                let mut stack: Vec<String> = Vec::new();
                let mut expressions = Vec::new();

           for op in op_reader {
    if let Ok(operator) = op {
        match operator {
            Operator::LocalGet { local_index } => {
                stack.push(format!("param{}", local_index));
            }
            
            Operator::I32Const { value } => {
                stack.push(value.to_string());
            }
            
            Operator::GlobalGet { global_index } => {
                stack.push(format!("global{}", global_index));
            }
            Operator::I32Add => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(format!("({} + {})", a, b));
                }
            }
            Operator::I32Sub => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(format!("({} - {})", a, b));
                }
            }
            Operator::I32Mul => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(format!("({} * {})", a, b));
                }
            }
            Operator::LocalSet { local_index } => {
                if let Some(popped_val) = stack.pop() {
                    expressions.push(format!("    let local{} = {};", local_index, popped_val));
                }
            }
            Operator::LocalTee { local_index } => {
                if let Some(popped_val) = stack.last() {
                    expressions.push(format!("    let local{} = {};", local_index, popped_val));
                }
            }
            
            Operator::GlobalSet { global_index } => {
                if let Some(popped_val) = stack.pop() {
                    expressions.push(format!("    global{} = {};", global_index, popped_val));
                }
            }
            Operator::End => {
                if let Some(final_expr) = stack.pop() {
                    expressions.push(format!("    return {};", final_expr));
                }
            }
            _ => {}
        }
    }
} 

                
                println!("fn {}(param0: i32, param1: i32) {{", func_name);
                if expressions.is_empty() {
                    println!("    // (No expressions or void function)");
                } else {
                    for expr in expressions {
                        println!("{}", expr);
                    }
                }
                println!("}}\n");
            }
        }
        _ => {
            println!("(Json / Html formats are not implemented for the real AST yet)");
        }
    }
}
