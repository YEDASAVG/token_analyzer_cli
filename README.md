# Token Analyzer

A command-line tool for analyzing and comparing tokenization across different Large Language Models (LLMs). Analyze token counts, estimate costs, and compare efficiency across GPT models.

## Features

- Analyze tokenization for individual GPT models
- Compare tokenization across multiple models simultaneously
- Calculate token-to-character ratios
- Estimate API costs based on token usage
- Support for GPT-4, GPT-4 Turbo, and GPT-3.5 Turbo

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo

### Build from source

```bash
git clone <repository-url>
cd token_analyzer
cargo build --release
```

## Usage

### Basic Analysis

Analyze a text file using the default model (GPT-4):

```bash
cargo run -- --file <FILE_PATH>
```

### Specify a Model

Analyze with a specific model:

```bash
cargo run -- --file <FILE_PATH> --model gpt-3.5-turbo
```

Available models:
- `gpt-4`
- `gpt-4-turbo`
- `gpt-3.5-turbo`

### Compare Models

Compare tokenization across all supported models:

```bash
cargo run -- --file <FILE_PATH> --compare
```

### Enable Visualization

Add the visualization flag (coming in future updates):

```bash
cargo run -- --file <FILE_PATH> --visualize
```

### Combined Options

```bash
cargo run -- --file <FILE_PATH> --compare --visualize
```

## Command-Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--file` | `-f` | Path to the text file to analyze (required) | - |
| `--model` | `-m` | Specify which model to use | `gpt-4` |
| `--compare` | `-c` | Compare across all models | `false` |
| `--visualize` | `-v` | Enable visualization (coming soon) | `false` |
| `--help` | `-h` | Print help information | - |

## Example Output

### Single Model Analysis

```
=== Token Analysis ===
Model: gpt-4
Characters: 838809
Tokens: 389852
Tokens/Char: 0.4648
Cost (input): $11.695560
```

### Model Comparison

```
=== Model Comparison ===
------------------------------------------------------------------------
Model                Tokens          Tokens/Char     Cost           
------------------------------------------------------------------------
gpt-4                389852          0.4648          $11.695560     
gpt-4-turbo          389852          0.4648          $3.898520      
gpt-3.5-turbo        389852          0.4648          $0.194926      
------------------------------------------------------------------------
```

## Technical Details

Built with Rust using the following dependencies:
- `clap` - Command-line argument parsing
- `anyhow` - Error handling
- `tiktoken-rs` - Tokenization library
- `colored` - Terminal output formatting

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
