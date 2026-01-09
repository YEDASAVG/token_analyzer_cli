use std::fs;
use colored::Colorize;
use clap::Parser;
use anyhow::Result;

mod analyzer;
mod comparator;
mod formatter;

#[derive(Parser)]
#[command(name = "token-analyzer")]
#[command(about = "Analyze and compare LLM tokenization")]

struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value = "gpt-4")]
    model: String,
    
    #[arg(short, long)]
    compare: bool,

    #[arg(short, long)]
    visualize: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();

    let text = fs::read_to_string(args.file)?;
    if args.compare{
        let result = comparator::compare_models(&text)?;
        formatter::display_comparison(&result);
    } else {
        let model = args.model;
        let analysis = analyzer::analyze_text(&text, &model)?;
        formatter::display_single(&analysis);
    }
    if args.visualize {
        println!("\n{}", "Visualization coming in future updates!".bright_cyan());
    }

    Ok(())
}
