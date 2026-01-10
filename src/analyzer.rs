use anyhow::{Result};
use tiktoken_rs::get_bpe_from_model;

pub struct TokenAnalysis {
    #[allow(unused)]
    pub text: String,
    pub model: String,
    pub char_count: usize,
    pub token_count: usize,
    pub token_per_char: f64,
    pub cost: f64,
}

pub fn analyze_text(text: &str, model:&str) -> Result<TokenAnalysis>{
    let bpe = get_bpe_from_model(model)?;
    let tokens = bpe.encode_with_special_tokens(text);

    let token_count = tokens.len();
    let char_count = text.chars().count();

    let token_per_char = token_count as f64 / char_count as f64;

    let price_per_1m = get_model_price(model);
    let cost = (token_count as f64 / 1_000_000.0) * price_per_1m;

    Ok(TokenAnalysis { text: text.to_string(), model: model.to_string(), char_count, token_count, token_per_char, cost })


}

pub fn get_model_price(model: &str) ->f64 {
    match model {
        "gpt-4" => 30.0,
        "gpt-4-turbo" => 10.0,
        "gpt-3.5-turbo" => 0.50,
        _ => 30.0
    }
}

#[cfg(test)]
mod tests{
    use crate::comparator::compare_models;

    use super::*;

    #[test]
    fn test_get_model_pricing() {
        assert_eq!(get_model_price("gpt-4"), 30.0);
        assert_eq!(get_model_price("gpt-4-turbo"), 10.0);
        assert_eq!(get_model_price("gpt-3.5-turbo"), 0.50);
        assert_eq!(get_model_price("unknown"), 30.0);
    }

    #[test]
    fn test_analyze_text() -> Result<()> {
        let text = "Hello World";
        let result = analyze_text(text, "gpt-4-turbo")?;

        assert_eq!(result.model, "gpt-4-turbo");
        assert_eq!(result.char_count, 11);
        assert!(result.token_count > 0);
        assert!(result.cost > 0.0);
        assert!(result.token_per_char > 0.0);

        Ok(())
    }

    #[test]
    fn test_compare_models() -> anyhow::Result<()> {
        let text = "Hellp world test";
        let result = compare_models(text)?;

        assert_eq!(result.analyses.len(), 3);
        
        let models: Vec<&str> = result.analyses.iter().map(|a| a.model.as_str()).collect();

        assert!(models.contains(&"gpt-4"));
        assert!(models.contains(&"gpt-4-turbo"));
        assert!(models.contains(&"gpt-3.5-turbo"));

        Ok(())
    }
}

