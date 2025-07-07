use crate::models::providers::{AiProvider, ModelName};
use crate::services::providers::{create_provider, AiProviderTrait};
use crate::error::{ConvComError, Result};
use anyhow::Result as AnyhowResult;

/// AI service for generating commit messages using multiple providers
pub struct AiService {
    providers: std::collections::HashMap<AiProvider, Box<dyn AiProviderTrait>>,
}

impl AiService {
    /// Create a new AI service instance with provider support
    pub fn new(groq_api_key: Option<String>, anthropic_api_key: Option<String>) -> Result<Self> {
        let mut providers = std::collections::HashMap::new();
        
        // Add Groq provider if API key is provided
        if let Some(key) = groq_api_key {
            let provider = create_provider(AiProvider::Groq, key)?;
            providers.insert(AiProvider::Groq, provider);
        }
        
        // Add Anthropic provider if API key is provided
        if let Some(key) = anthropic_api_key {
            let provider = create_provider(AiProvider::Anthropic, key)?;
            providers.insert(AiProvider::Anthropic, provider);
        }
        
        if providers.is_empty() {
            return Err(ConvComError::ConfigError(
                "At least one AI provider API key must be provided".to_string()
            ));
        }

        Ok(Self { providers })
    }

    /// Create a new AI service instance with Groq only (backward compatibility)
    pub fn new_groq_only(api_key: String) -> Result<Self> {
        Self::new(Some(api_key), None)
    }

    /// Generate a commit message using the specified model
    pub async fn generate_commit_message(&self, prompt: String, model: ModelName) -> AnyhowResult<String> {
        let provider_type = model.provider();
        
        let provider = self.providers.get(&provider_type)
            .ok_or_else(|| ConvComError::ConfigError(
                format!("Provider {} is not configured. Please provide API key for this provider.", provider_type)
            ))?;

        let result = provider.generate_message(prompt, model).await?;
        Ok(result)
    }

    /// Get list of available providers
    pub fn available_providers(&self) -> Vec<AiProvider> {
        self.providers.keys().copied().collect()
    }

    /// Check if a specific provider is available
    pub fn has_provider(&self, provider: AiProvider) -> bool {
        self.providers.contains_key(&provider)
    }

    /// Get models available for configured providers
    pub fn available_models(&self) -> Vec<ModelName> {
        let mut models = Vec::new();
        
        for provider_type in self.providers.keys() {
            match provider_type {
                AiProvider::Groq => {
                    models.extend_from_slice(&[
                        ModelName::Allam27B,
                        ModelName::CompoundBeta,
                        ModelName::CompoundBetaMini,
                        ModelName::DeepSeekR1DistillLlama70B,
                        ModelName::Gemma29BIT,
                        ModelName::Llama318BInstant,
                        ModelName::Llama3370BVersatile,
                        ModelName::Llama370B8192,
                        ModelName::Llama38B8192,
                        ModelName::MetaLlama4Maverick17B128E,
                        ModelName::MetaLlama4Scout17B16E,
                        ModelName::MetaLlama4Maverick17B128E,
                        ModelName::MetaLlamaPromptGuard222M,
                        ModelName::MetaLlamaPromptGuard286M,
                        ModelName::MistralSaba24B,
                        ModelName::QwenQWQ32B,
                        ModelName::Qwen332B,
                    ]);
                }
                AiProvider::Anthropic => {
                    models.extend_from_slice(&[
                        ModelName::Claude4Sonnet,
                        ModelName::Claude35Sonnet,
                        ModelName::Claude35Haiku,
                        ModelName::Claude3Opus,
                        ModelName::Claude3Sonnet,
                        ModelName::Claude3Haiku,
                    ]);
                }
            }
        }
        
        models
    }
}
