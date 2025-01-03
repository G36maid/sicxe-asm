# SIC/XE Assembler

A fully-featured SIC/XE assembler implementation.

## Features

- **Instruction & Directive Support**: Handles all SIC/XE instructions and directives.
- **Control Sections**: Enables separation of code into logical sections.
- **Program Blocks**: Supports program block configurations.
- **Literals**: Includes literal handling.
- **Symbol-Defining Directives**: Implements `EQU` for symbol definition.
- **Syntax & Semantic Checking**: Provides basic syntax validation and semantic error detection.

## Parallel Assemble (Experimental)

Introducing the experimental `merge_texts_parallel` feature, which leverages multi-threading using the Rayon library to optimize performance during the merging and processing of text records. This feature is designed to improve efficiency for large datasets while ensuring correctness.

## Usage

The assembler is built using Rust. Ensure you have the latest stable version installed before proceeding.

### Building the Assembler

```bash
cargo build --release
```

### Command Overview

```bash
./target/release/sicxe-asm help              
Usage: sicxe-asm [COMMAND]

Commands:
  assemble  Assemble the source code
  optimize  Optimize the object code
  dir       Optimize all object files in the directory
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Running the Assembler

To see usage instructions:

```bash
./target/release/sicxe-asm -h
```

To assemble a source file:

```bash
./target/release/sicxe-asm assemble <source-file>
```

## Architecture

The assembler is composed of four key components:

1. **Assembler**
    1. **Tokenizer**(crates: frame): Removes comments and splits lines into tokens.
    2. **Parser**(crates: frame): Converts tokens into structured **Frames**.
    3. **Transformers**(crates:frameformer): Transforms a sequence of frames to resolve directives and symbols.
2. **Optimizer**: Compresses the object code layout for compactness.

### Frames

Frames represent the core processing units in the assembler. They come in three types:

- **Instruction**: Represents commands like `LDA`, `J`, `JSUB`.
- **Directive**: Covers operations such as `START`, `END`, `BYTE`, `WORD`.
- **ObjectRecord**: Encodes object records (`T`, `M`, `E`).

### Transformers

- **Section Splitter**: Divides the program into control sections.
- **Block Rearranger**: Arranges frame layout based on program blocks.
- **Literal Dumper**: Converts literals into `BYTE` directives.
- **Symbol Resolver**: Resolves symbols and substitutes their addresses.
- **Translator**: Converts frames into object records.

### Optimizer

The optimizer refines object code layout, ensuring minimal size without compromising functionality.

### Parallel Merge

The `merge_texts_parallel` function leverages Rayon for concurrent processing of text records. This improves performance, especially for large programs, by distributing the workload across multiple threads. The feature remains experimental and should be tested in diverse scenarios to ensure robustness.

## Crate Dependency

This project utilizes the [sicxe crate](https://crates.io/crates/sicxe), which provides the core functionality for SIC/XE instruction parsing and processing. The source code for the `sicxe` crate is available on GitHub at [JacobLinCool/sicxe](https://github.com/JacobLinCool/sicxe). The `sicxe` crate is distributed under the AGPL-3.0 License.

## Copyright Claim

The implementation of the SIC/XE assembler in this project includes contributions from both open-source and original development. Specifically:

- **My Contribution**: I implemented the optimization module, which refines the layout of the object code to make it as compact as possible. This involved designing and coding the optimization logic to ensure that the object programs generated by the assembler are space-efficient while adhering to SIC/XE specifications.

- **Open-Source Contributions**: The assembler leverages the open-source `sicxe` crate, developed by [JacobLinCool](https://github.com/JacobLinCool/sicxe). This crate provides foundational functionalities, including parsing SIC/XE instructions and directives, which greatly accelerated the development process.

## Acknowledgment

While I developed the optimization component independently, the use of the `sicxe` crate for core assembler functionalities is essential to the overall implementation. This project is built upon Jacob Lin's work and distributed under the same [AGPL-3.0 License](https://www.gnu.org/licenses/agpl-3.0.en.html), in compliance with the open-source license terms.
