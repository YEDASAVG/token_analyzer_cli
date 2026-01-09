use crate::analyzer::{TokenAnalysis, analyze_text};

pub struct ComparisonResult {
    pub analyses: Vec<TokenAnalysis>,
}

pub fn compare_models(text: &str) -> anyhow::Result<ComparisonResult> {
    let models = vec!["gpt-4", "gpt-4-turbo", "gpt-3.5-turbo"];

    let mut analyses = Vec::new();
    for model in models {
        match analyze_text(text, model) {
            Ok(analysis) => analyses.push(analysis),
            Err(_) => continue,
        }
    }
    Ok(ComparisonResult { analyses })
}