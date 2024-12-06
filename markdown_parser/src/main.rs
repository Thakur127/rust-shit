use markdown_parser::markdown;
use std::{env, fs, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <markdown file> [<output file>]", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // read from file
    let data = fs::read_to_string(file_path)?;

    let markdown_input = markdown::Markdown { content: data };

    // convert to html
    let html_output = markdown_input.to_html();
    println!("{}", html_output);

    // write to file
    if args.len() > 2 {
        let output_path = &args[2];
        fs::write(output_path, html_output)?;
    }

    return Ok(());
}
