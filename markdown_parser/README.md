# Markdown Parser

A simple(shitty) Markdown parser written in Rust. This is a basic project created as a learning exercise to understand Rust and how to process text files.

## Features

- Converts basic Markdown syntax to HTML.
- Supports headers, bold text, italic text, links, and images.
- Accepts input from a Markdown file and outputs HTML.

## Requirements

- Rust (1.50 or later) installed. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

## Usage

1. **Clone the repository**:
   First, clone this repository to your local machine.

   ```bash
   git clone https://github.com/Thakur127/rust-shit.git
   cd rust-shit/markdown_parser
   ```

2. **Run the project**:

   ```bash
   cargo run -- <input_file> [<output_file>]
   ```

   - `<input_file>`: The path to the input Markdown file.
   - `<output_file>` (optional): The path to the output HTML file. If not provided, the output will be printed to the console.

3. **Example**:

   ```bash
   cargo run -- test.md output.html
   ```

## Limitations

- This parser only handles basic Markdown syntax (headers, bold, italics, links, and images).
- It does not support advanced Markdown features like tables, code blocks, or nested lists.
- The formatting might not be perfect and could need further refinement.
