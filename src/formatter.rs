use colored::*;
use crate::analyzer::TokenAnalysis;
use crate::comparator::ComparisonResult;


pub fn display_single(analysis: &TokenAnalysis) {
    println!("\n{}", "=== Token Analysis ===".bright_blue().bold());
    println!("{}: {}", "Model".bright_yellow(), analysis.model);
    println!("{}: {}", "Characters".bright_yellow(), analysis.char_count);
    println!("{}: {}", "Tokens".bright_yellow(), analysis.token_count);
    println!("{}: {:.4}", "Tokens/Char".bright_yellow(), analysis.token_per_char);
    println!("{}: ${:.6}", "Cost (input)".bright_yellow(), analysis.cost); 
}

pub fn display_comparison(result: &ComparisonResult) {
    println!("\n{}", "=== Model Comparison ===".bright_blue().bold());
    println!("{}", "-".repeat(80));

    println!(
        "{:<20} {:<15} {:<15} {:<20}",
        "Model".bright_yellow(),
        "Tokens".bright_yellow(),
        "Tokens/Char".bright_yellow(),
        "Cost".bright_yellow(),
    );
    println!("{}", "-".repeat(80));

    for analysis in &result.analyses {
        println!(
            "{:<20} {:<15} {:<15.4} ${:<19.6}",
            analysis.model.bright_green(),
            analysis.token_count,
            analysis.token_per_char,
            analysis.cost
        );
    }
    println!("{}", "-".repeat(80));
}