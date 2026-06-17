mod decompile;
mod definition;
mod inspectors;

use clap::Parser;
use definition::Args;

fn main() {
    let args = Args::parse();

    if args.verbose {
        println!("INFO: Loading and parsing target file: {}", args.wat_path);
    }

    
    let (function_bodies, function_names) = inspectors::run_inspect(&args);

    
    decompile::run_decompile(&args, function_bodies, function_names);
}
