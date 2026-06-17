use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
    Html,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Wasm/WAT Algorithm Visualizer and Analyzer"
)]
pub struct Args {
    /// Target WAT or Wasm file path
    pub wat_path: String,

    /// Output format (text, json, html)
    #[arg(long, value_enum, default_value = "text")]
    pub format: OutputFormat,

    /// Verbose output (show internal stack states and details)
    #[arg(short, long)]
    pub verbose: bool,

    /// Skip Data-Flow Analysis (DFA) and control flow graphing
    #[arg(long)]
    pub no_dfa: bool,
}
