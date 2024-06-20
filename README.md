# Rust Compiler 
# Overview

Welcome to the Rust Compiler project! This compiler is designed to transform source code written in a custom programming language into executable code. It leverages the safety, speed, and concurrency features of Rust to deliver a robust and efficient compilation process.

# Features
1. Lexical Analysis: Tokenizes the source code into meaningful symbols.
2. Syntax Analysis: Parses tokens into an abstract syntax tree (AST).
3.Semantic Analysis: Ensures the program is semantically correct.
4. Intermediate Representation (IR): Generates an intermediate representation for optimization.
5. Optimization: Optimizes the IR for better performance.
6. Code Generation: Translates the optimized IR into target machine code.
7. Error Handling: Provides comprehensive error messages for debugging.


# Requirements
Rust (latest stable version) - Install Rust
Installation


Clone the repository:
git clone https://github.com/yourusername/rust-compiler.git
cd rust-compiler


Build the project:
cargo build --release
Usage
To compile a source file:


cargo run -- <source_file>
Replace <source_file> with the path to your source file.

# Project Structure
src/
1. lexer.rs: Contains the lexical analysis logic.
2. parser.rs: Contains the syntax analysis logic.
3. semantic.rs: Handles semantic analysis.
4. ir.rs: Defines the intermediate representation.
5. optimizer.rs: Implements optimization techniques.
6. codegen.rs: Manages code generation.
7. error.rs: Handles error reporting and management.
8 .main.rs: The entry point of the compiler.

# Example
To compile a file named example.lang, run:

cargo run -- example.lang

# Contirbitor
1. Ayaan Hasan
