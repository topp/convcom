use serde::{Deserialize, Serialize};
use std::fmt;

/// AI Provider selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, clap::ValueEnum)]
pub enum AiProvider {
    /// Groq (default)
    #[value(name = "groq")]
    Groq,
    
    /// Anthropic Claude
    #[value(name = "anthropic")]
    Anthropic,
}

impl Default for AiProvider {
    fn default() -> Self {
        AiProvider::Groq
    }
}

impl fmt::Display for AiProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AiProvider::Groq => write!(f, "groq"),
            AiProvider::Anthropic => write!(f, "anthropic"),
        }
    }
}

/// Available AI models across all providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ModelName {
    // Groq Models
    /// Allam 2 7B model (Groq)
    #[value(name = "allam-2-7b")]
    Allam27B,
    
    /// Compound Beta model (Groq)
    #[value(name = "compound-beta")]
    CompoundBeta,
    
    /// Compound Beta Mini model (Groq)
    #[value(name = "compound-beta-mini")]
    CompoundBetaMini,
    
    /// DeepSeek R1 Distill Llama 70B (Groq)
    #[value(name = "deepseek-r1-distill-llama-70b")]
    DeepSeekR1DistillLlama70B,
    
    /// Gemma2 9B Instruct model (Groq)
    #[value(name = "gemma2-9b-it")]
    Gemma29BIT,
    
    /// Llama 3.1 8B Instant model (Groq)
    #[value(name = "llama-3.1-8b-instant")]
    Llama318BInstant,
    
    /// Llama 3.3 70B Versatile model (Groq) (default)
    #[value(name = "llama-3.3-70b-versatile")]
    Llama3370BVersatile,
    
    /// Llama3 70B 8192 context model (Groq)
    #[value(name = "llama3-70b-8192")]
    Llama370B8192,
    
    /// Llama3 8B 8192 context model (Groq)
    #[value(name = "llama3-8b-8192")]
    Llama38B8192,
    
    /// Meta Llama 4 Maverick 17B 128E Instruct (Groq)
    #[value(name = "meta-llama/llama-4-maverick-17b-128e-instruct")]
    MetaLlama4Maverick17B128E,
    
    /// Meta Llama 4 Scout 17B 16E Instruct (Groq)
    #[value(name = "meta-llama/llama-4-scout-17b-16e-instruct")]
    MetaLlama4Scout17B16E,
    
    /// Meta Llama Guard 4 12B (Groq)
    #[value(name = "meta-llama/llama-guard-4-12b")]
    MetaLlamaGuard412B,
    
    /// Meta Llama Prompt Guard 2 22M (Groq)
    #[value(name = "meta-llama/llama-prompt-guard-2-22m")]
    MetaLlamaPromptGuard222M,
    
    /// Meta Llama Prompt Guard 2 86M (Groq)
    #[value(name = "meta-llama/llama-prompt-guard-2-86m")]
    MetaLlamaPromptGuard286M,
    
    /// Mistral Saba 24B model (Groq)
    #[value(name = "mistral-saba-24b")]
    MistralSaba24B,
    
    /// Qwen QWQ 32B model (Groq)
    #[value(name = "qwen-qwq-32b")]
    QwenQWQ32B,
    
    /// Qwen3 32B model (Groq)
    #[value(name = "qwen/qwen3-32b")]
    Qwen332B,

    // Anthropic Models
    /// Claude 3.5 Sonnet (Anthropic)
    #[value(name = "claude-3-5-sonnet-20241022")]
    Claude35Sonnet,
    
    /// Claude 3.5 Haiku (Anthropic)
    #[value(name = "claude-3-5-haiku-20241022")]
    Claude35Haiku,
    
    /// Claude 3 Opus (Anthropic)
    #[value(name = "claude-3-opus-20240229")]
    Claude3Opus,
    
    /// Claude 3 Sonnet (Anthropic)
    #[value(name = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    
    /// Claude 3 Haiku (Anthropic)
    #[value(name = "claude-3-haiku-20240307")]
    Claude3Haiku,

    /// Claude 4 Sonnet (Anthropic)
    #[value(name = "claude-sonnet-4-20250514")]
    Claude4Sonnet,
}

impl ModelName {
    /// Get the string value for the API
    pub fn as_str(&self) -> &'static str {
        match self {
            // Groq models
            ModelName::Allam27B => "allam-2-7b",
            ModelName::CompoundBeta => "compound-beta",
            ModelName::CompoundBetaMini => "compound-beta-mini",
            ModelName::DeepSeekR1DistillLlama70B => "deepseek-r1-distill-llama-70b",
            ModelName::Gemma29BIT => "gemma2-9b-it",
            ModelName::Llama318BInstant => "llama-3.1-8b-instant",
            ModelName::Llama3370BVersatile => "llama-3.3-70b-versatile",
            ModelName::Llama370B8192 => "llama3-70b-8192",
            ModelName::Llama38B8192 => "llama3-8b-8192",
            ModelName::MetaLlama4Maverick17B128E => "meta-llama/llama-4-maverick-17b-128e-instruct",
            ModelName::MetaLlama4Scout17B16E => "meta-llama/llama-4-scout-17b-16e-instruct",
            ModelName::MetaLlamaGuard412B => "meta-llama/llama-guard-4-12b",
            ModelName::MetaLlamaPromptGuard222M => "meta-llama/llama-prompt-guard-2-22m",
            ModelName::MetaLlamaPromptGuard286M => "meta-llama/llama-prompt-guard-2-86m",
            ModelName::MistralSaba24B => "mistral-saba-24b",
            ModelName::QwenQWQ32B => "qwen-qwq-32b",
            ModelName::Qwen332B => "qwen/qwen3-32b",
            
            // Anthropic models
            ModelName::Claude4Sonnet => "claude-sonnet-4-20250514",
            ModelName::Claude35Sonnet => "claude-3-5-sonnet-20241022",
            ModelName::Claude35Haiku => "claude-3-5-haiku-20241022",
            ModelName::Claude3Opus => "claude-3-opus-20240229",
            ModelName::Claude3Sonnet => "claude-3-sonnet-20240229",
            ModelName::Claude3Haiku => "claude-3-haiku-20240307",
        }
    }

    /// Get the provider this model belongs to
    pub fn provider(&self) -> AiProvider {
        match self {
            // Groq models
            ModelName::Allam27B
            | ModelName::CompoundBeta
            | ModelName::CompoundBetaMini
            | ModelName::DeepSeekR1DistillLlama70B
            | ModelName::Gemma29BIT
            | ModelName::Llama318BInstant
            | ModelName::Llama3370BVersatile
            | ModelName::Llama370B8192
            | ModelName::Llama38B8192
            | ModelName::MetaLlama4Maverick17B128E
            | ModelName::MetaLlama4Scout17B16E
            | ModelName::MetaLlamaGuard412B
            | ModelName::MetaLlamaPromptGuard222M
            | ModelName::MetaLlamaPromptGuard286M
            | ModelName::MistralSaba24B
            | ModelName::QwenQWQ32B
            | ModelName::Qwen332B => AiProvider::Groq,
            
            // Anthropic models
            ModelName::Claude4Sonnet
            | ModelName::Claude35Sonnet
            | ModelName::Claude35Haiku
            | ModelName::Claude3Opus
            | ModelName::Claude3Sonnet
            | ModelName::Claude3Haiku => AiProvider::Anthropic,
        }
    }
}

impl Default for ModelName {
    fn default() -> Self {
        ModelName::Llama3370BVersatile
    }
}

impl fmt::Display for ModelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_provider_mapping() {
        assert_eq!(ModelName::Llama3370BVersatile.provider(), AiProvider::Groq);
        assert_eq!(ModelName::Claude35Sonnet.provider(), AiProvider::Anthropic);
        assert_eq!(ModelName::Claude3Haiku.provider(), AiProvider::Anthropic);
    }

    #[test]
    fn test_provider_display() {
        assert_eq!(AiProvider::Groq.to_string(), "groq");
        assert_eq!(AiProvider::Anthropic.to_string(), "anthropic");
    }

    #[test]
    fn test_default_values() {
        assert_eq!(AiProvider::default(), AiProvider::Groq);
        assert_eq!(ModelName::default(), ModelName::Llama3370BVersatile);
    }
}
