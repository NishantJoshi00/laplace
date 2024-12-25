# Laplace

A minimalist lambda calculus interpreter written in Rust. Laplace provides a simple yet powerful way to evaluate lambda expressions with support for variable binding, function application, and expression evaluation.

## Description

Laplace is a lambda calculus implementation that:

- Parses and evaluates untyped lambda calculus expressions
- Supports variable binding through `let` statements
- Includes a CLI interface for direct interaction
- Provides detailed error reporting with color-coded output
- Features a lexer and parser for lambda expressions using nom
- Implements beta-reduction for function application

## Installation

1. Ensure you have Rust and Cargo installed on your system. If not, install them from [rustup.rs](https://rustup.rs)

2. Clone the repository:

   ```bash
   git clone <repository-url>
   cd laplace
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Basic Usage

1. Create a lambda calculus file with `.l` extension, for example `script.l`:

   ```
   let TRUE = \x.\y.x
   let MAIN = \x.(x TRUE)
   ```

2. Run the interpreter:
   ```bash
   cargo run --release --bin dirty -- script.l
   ```

### File Format

Lambda expressions follow this syntax:

- Functions are defined using `\` or `λ` followed by a variable and `.`
- Variables are lowercase letters
- Function applications are wrapped in parentheses
- Definitions use `let` keyword with uppercase identifiers

Example:

```
let TRUE = \x.\y.x
let APPLY = \x.\y.y
let MAIN = \x.(x TRUE)
```

### Development Mode

For development with auto-reload:

```bash
make watch
```

## Features

- **Parser Combinators**: Uses nom for robust parsing of lambda expressions
- **Error Handling**: Comprehensive error reporting with color-eyre integration
- **Token System**: Sophisticated token management for variable scoping
- **Beta Reduction**: Implements proper beta reduction for function evaluation
- **Development Tools**: Includes watch mode for rapid development
- **Modular Architecture**: Clean separation between lexer, evaluator, and runtime
- **CLI Interface**: Easy-to-use command line interface with clap
- **Color Output**: Syntax highlighting for better readability

## Contributing guidelines

1. Create an issue for any feature or bug fix you want to work on
2. Tag your issues appropriately with [BUG], [FEATURE], etc.
3. Do not work on issues that are already assigned
4. Make sure your code follows the existing style and includes appropriate tests
5. Run the test suite before submitting your changes
