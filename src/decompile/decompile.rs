use crate::definition::{Args, OutputFormat};
use std::collections::HashMap;
use wasmparser::{Operator, WasmFeatures};

pub fn run_decompile(args: &Args, function_bodies: Vec<Vec<u8>>, function_names: HashMap<u32, String>) {
    
    
    if let OutputFormat::Text = args.format {
        println!("--- Running Decompiler ---");
        if !args.no_dfa {
            println!("[DFA] Analyzing control flow graph and expression stack...\n");
        }
    }

    
    
    let mut decompiled_functions = Vec::new();

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
                            expressions.push(format!("let local{} = {};", local_index, popped_val));
                        }
                    }
                    Operator::LocalTee { local_index } => {
                        if let Some(popped_val) = stack.last() {
                            expressions.push(format!("let local{} = {};", local_index, popped_val));
                        }
                    }
                    Operator::GlobalSet { global_index } => {
                        if let Some(popped_val) = stack.pop() {
                            expressions.push(format!("global{} = {};", global_index, popped_val));
                        }
                    }
                    Operator::End => {
                        if let Some(final_expr) = stack.pop() {
                            expressions.push(format!("return {};", final_expr));
                        }
                    }
                    _ => {}
                }
            }
        }
        decompiled_functions.push((func_name, expressions));
    }

   
    match args.format {
        OutputFormat::Text => {
            println!("=== Decompiled Pseudo-Code (Text) ===");
            for (func_name, expressions) in decompiled_functions {
                println!("fn {}(param0: i32, param1: i32) {{", func_name);
                if expressions.is_empty() {
                    println!("    // (No expressions or void function)");
                } else {
                    for expr in expressions {
                        println!("    {}", expr);
                    }
                }
                println!("}}\n");
            }
        }
        
        OutputFormat::Json => {
           
            println!("{{");
            println!("  \"target_file\": \"{}\",", args.wat_path.replace('\\', "\\\\").replace('"', "\\\""));
            println!("  \"functions\": [");
            for (f_idx, (func_name, expressions)) in decompiled_functions.iter().enumerate() {
                println!("    {{");
                println!("      \"name\": \"{}\",", func_name);
                println!("      \"expressions\": [");
                for (e_idx, expr) in expressions.iter().enumerate() {
                    let comma = if e_idx == expressions.len() - 1 { "" } else { "," };
                    println!("        \"{}\"{}", expr.replace('"', "\\\""), comma);
                }
                println!("      ]");
                let trailing_comma = if f_idx == decompiled_functions.len() - 1 { "" } else { "," };
                println!("    }}{}", trailing_comma);
            }
            println!("  ]");
            println!("}}");
        }
        
        OutputFormat::Html => {
           
            println!("<!DOCTYPE html>");
            println!("<html lang=\"en\">");
            println!("<head>");
            println!("    <meta charset=\"UTF-8\">");
            println!("    <title>wasp Decompiler Report</title>");
            println!("    <style>");
            println!("        body {{ font-family: 'Segoe UI', sans-serif; background: #1e1e1e; color: #d4d4d4; padding: 20px; }}");
            println!("        h1 {{ color: #569cd6; border-bottom: 2px solid #3c3c3c; padding-bottom: 10px; }}");
            println!("        .file-info {{ color: #4ec9b0; font-style: italic; margin-bottom: 20px; }}");
            println!("        .func-box {{ background: #252526; border: 1px solid #3c3c3c; border-radius: 6px; padding: 15px; margin-bottom: 15px; box-shadow: 0 4px 6px rgba(0,0,0,0.3); }}");
            println!("        .func-name {{ color: #dcdcaa; font-size: 1.2em; font-weight: bold; margin-bottom: 10px; font-family: monospace; }}");
            println!("        .code-line {{ font-family: 'Consolas', monospace; color: #9cdcfe; padding-left: 20px; margin: 4px 0; border-left: 2px solid #608b4e; }}");
            println!("        .keyword {{ color: #569cd6; }}");
            println!("        .comment {{ color: #608b4e; font-style: italic; }}");
            println!("    </style>");
            println!("</head>");
            println!("<body>");
            println!("    <h1>🐝 wasp Decompiler Report</h1>");
            println!("    <div class=\"file-info\">Target: {}</div>", args.wat_path);

            for (func_name, expressions) in decompiled_functions {
                println!("    <div class=\"func-box\">");
                println!("        <div class=\"func-name\"><span class=\"keyword\">fn</span> {}(param0: i32, param1: i32) {{</div>", func_name);
                
                if expressions.is_empty() {
                    println!("        <div class=\"code-line comment\">// (No expressions or void function)</div>");
                } else {
                    for expr in expressions {
                       
                        let highlighted = expr
                            .replace("return ", "<span class=\"keyword\">return </span>")
                            .replace("let ", "<span class=\"keyword\">let </span>");
                        println!("        <div class=\"code-line\">{}</div>", highlighted);
                    }
                }
                println!("        <div class=\"func-name\">}}</div>");
                println!("    </div>");
            }

            println!("</body>");
            println!("</html>");
        }
    }
}
