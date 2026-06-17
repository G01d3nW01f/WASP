# wasp 🐝

`wasp` (Wasm/WAT Algorithm Visualizer and Analyzer) is a Rust-based tool designed to read WebAssembly (Wasm) and WAT (WebAssembly Text Format) files. It tracks low-level stack machine instructions and automatically decompiles/abstracts them into high-level, human-readable pseudo-code.

---

## 🚀 Key Features

* **Real-time Stack Simulation:** Tracks stack-based operations like `local.get` and `i32.add` to reconstruct them into intuitive mathematical expressions (e.g., `(param0 + param1)`).
* **Function Resolution via Exports:** Parses the `ExportSection` to automatically map internal Wasm function indices to their actual exported names (e.g., `fn getGlobal()`).
* **Local/Global Variable Tracking:** Decodes variable mutation instructions like `local.set` and `global.set` into clean variable assignments (`let local0 = ...`, `global0 = ...`).
* **Modular Architecture:** Perfectly decouples CLI parsing, structure inspection, and decompilation, making it incredibly easy to add support for new Wasm operators.

---

## 🛠️ Directory Structure

The project is structured into clean, isolated modules:

```
.
├── Cargo.toml
└── src
    ├── main.rs            # Entry point & subcommand/flag routing
    ├── definition.rs      # CLI Argument definitions using clap (Derive style)
    ├── inspectors/        # Wasm structure parsing & basic info extraction
    │   ├── mod.rs
    │   └── inspector.rs
    └── decompile/         # Core decompilation logic using a virtual stack
        ├── mod.rs
        └── decompile.rs
```

---

## 📦 Dependencies

This project leverages the powerful and robust components of the Rust Wasm ecosystem:

* `clap` (v4.5) - Robust command-line argument parser.
* `wat` - Seamlessly translates WAT text into parseable Wasm binary bytes.
* `wasmparser` - The industry-standard library for fast and safe Wasm binary parsing.

---

## 💻 Usage

### 1. Build the Project
Compile the binary in release mode:

```bash
cargo build --release
```

### 2. Run the Analyzer
Pass the path of the `.wat` or `.wasm` file you want to reverse-engineer:

```bash
target/release/wasp path/to/your_file.wat
```

### 💡 Example Workflow

Given a `global.wat` file that manipulates a global state:

```wat
(module
  (global $g (import "js" "global") (mut i32))
  (func (export "getGlobal") (result i32)
    (global.get $g)
  )
  (func (export "incGlobal")
    (global.set $g (i32.add (global.get $g) (i32.const 1)))
  )
)
```

**Running the tool will yield:**

```text
=== Wasm Structure Report ===
Total Functions defined: 2
  - Function #0: Size 4 bytes
  - Function #1: Size 9 bytes
Imports: 0

--- Running Decompiler ---
[DFA] Analyzing control flow graph and expression stack...

=== Decompiled Pseudo-Code (Text) ===
fn getGlobal(param0: i32, param1: i32) {
    return global0;
}

fn incGlobal(param0: i32, param1: i32) {
    global0 = (global0 + 1);
}
```

---

## 🛠️ Command-Line Options

Based on `definition.rs`, you can tweak the analyzer using the following flags:

```bash
# Enable verbose internal logging & raw stack traces
target/release/wasp target.wat --verbose

# Skip Data-Flow Analysis (DFA)
target/release/wasp target.wat --no-dfa

# Specify output format (for future HTML/JSON visualization extensions)
target/release/wasp target.wat --format json
```

---

## 🤝 Contributing & Extending

`wasp` grows smarter with every Wasm operator it learns! If you want to add support for more instructions, simply jump into `src/decompile/decompile.rs` and expand the `match operator` block:

```rust
// Example: Adding support for subtraction
Operator::I32Sub => {
    if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
        stack.push(format!("({} - {})", a, b));
    }
}
```

Feel free to open Issues or Pull Requests for bug reports or feature requests (like rendering the CFG to an interactive HTML graph)!
```
