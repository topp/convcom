use crate::error::{ConvComError, Result};
use crate::models::ai::{ChatCompletionRequest, ChatCompletionResponse, ChatMessage};
use crate::models::providers::{AiProvider, ModelName};
use async_trait::async_trait;
use regex::Regex;
use reqwest::Client;
use serde_json::{json, Value};
use std::time::Duration;

/// Trait for AI providers
#[async_trait]
pub trait AiProviderTrait: Send + Sync {
    async fn generate_message(&self, prompt: String, model: ModelName) -> Result<String>;
    fn clean_output(&self, raw_output: &str) -> String;
}

/// Groq AI Provider implementation
pub struct GroqProvider {
    client: Client,
    api_key: String,
}

impl GroqProvider {
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ConvComError::HttpClientError(e.to_string()))?;

        Ok(Self { client, api_key })
    }

    fn build_chat_request(&self, prompt: String, model: ModelName) -> ChatCompletionRequest {
        let messages = vec![
            ChatMessage::system(
                "You are a helpful AI assistant that generates conventional commit messages.",
            ),
            ChatMessage::user(prompt),
        ];

        ChatCompletionRequest {
            model: model.as_str().to_string(),
            messages,
            temperature: 0.5,
            max_tokens: 1024,
        }
    }

    async fn make_groq_request(
        &self,
        request: ChatCompletionRequest,
    ) -> Result<ChatCompletionResponse> {
        let url = "https://api.groq.com/openai/v1/chat/completions";

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ConvComError::ApiRequestError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(ConvComError::ApiError {
                status_code: status.as_u16(),
                message: error_text,
            });
        }

        let chat_response: ChatCompletionResponse = response
            .json()
            .await
            .map_err(|e| ConvComError::ResponseParseError(e.to_string()))?;

        Ok(chat_response)
    }
}

#[async_trait]
impl AiProviderTrait for GroqProvider {
    async fn generate_message(&self, prompt: String, model: ModelName) -> Result<String> {
        if model.provider() != AiProvider::Groq {
            return Err(ConvComError::ConfigError(format!(
                "Model {} is not supported by Groq provider",
                model
            )));
        }

        let request = self.build_chat_request(prompt, model);
        let response = self.make_groq_request(request).await?;

        let raw_output = response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or(ConvComError::EmptyResponseError)?;

        Ok(self.clean_output(&raw_output))
    }

    fn clean_output(&self, raw_output: &str) -> String {
        // Remove thinking tags (similar to Python's regex)
        let thinking_regex = Regex::new(r"(?s)<think>.*?</think>").unwrap();
        let cleaned = thinking_regex.replace_all(raw_output, "");

        // Remove any remaining thinking tags
        let cleaned = cleaned.replace("<think>", "").replace("</think>", "");

        // Trim leading whitespace
        cleaned.trim_start().to_string()
    }
}

/// Anthropic AI Provider implementation
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ConvComError::HttpClientError(e.to_string()))?;

        Ok(Self { client, api_key })
    }

    async fn make_anthropic_request(&self, prompt: String, model: ModelName) -> Result<String> {
        let url = "https://api.anthropic.com/v1/messages";

        let request_body = json!({
            "model": model.as_str(),
            "max_tokens": 1024,
            "temperature": 0.5,
            "system": "You are a helpful AI assistant that generates conventional commit messages.",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let response = self
            .client
            .post(url)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ConvComError::ApiRequestError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(ConvComError::ApiError {
                status_code: status.as_u16(),
                message: error_text,
            });
        }

        let response_json: Value = response
            .json()
            .await
            .map_err(|e| ConvComError::ResponseParseError(e.to_string()))?;

        // Extract content from Anthropic response format
        let content = response_json
            .get("content")
            .and_then(|content| content.as_array())
            .and_then(|arr| arr.first())
            .and_then(|obj| obj.get("text"))
            .and_then(|text| text.as_str())
            .ok_or(ConvComError::EmptyResponseError)?;

        Ok(content.to_string())
    }
}

#[async_trait]
impl AiProviderTrait for AnthropicProvider {
    async fn generate_message(&self, prompt: String, model: ModelName) -> Result<String> {
        if model.provider() != AiProvider::Anthropic {
            return Err(ConvComError::ConfigError(format!(
                "Model {} is not supported by Anthropic provider",
                model
            )));
        }

        let raw_output = self.make_anthropic_request(prompt, model).await?;
        Ok(self.clean_output(&raw_output))
    }

    fn clean_output(&self, raw_output: &str) -> String {
        // Remove thinking tags (similar to Python's regex)
        let thinking_regex = Regex::new(r"(?s)<thinking>.*?</thinking>").unwrap();
        let cleaned = thinking_regex.replace_all(raw_output, "");

        // Remove any remaining thinking tags
        let cleaned = cleaned.replace("<thinking>", "").replace("</thinking>", "");

        // Trim leading whitespace
        cleaned.trim_start().to_string()
    }
}

/// Factory function to create the appropriate provider
pub fn create_provider(provider: AiProvider, api_key: String) -> Result<Box<dyn AiProviderTrait>> {
    match provider {
        AiProvider::Groq => {
            let groq_provider = GroqProvider::new(api_key)?;
            Ok(Box::new(groq_provider))
        }
        AiProvider::Anthropic => {
            let anthropic_provider = AnthropicProvider::new(api_key)?;
            Ok(Box::new(anthropic_provider))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_groq_provider_creation() {
        let provider = GroqProvider::new("test_key".to_string());
        assert!(provider.is_ok());
    }

    #[test]
    fn test_anthropic_provider_creation() {
        let provider = AnthropicProvider::new("test_key".to_string());
        assert!(provider.is_ok());
    }

    #[test]
    fn test_provider_factory() {
        let groq_provider = create_provider(AiProvider::Groq, "test_key".to_string());
        assert!(groq_provider.is_ok());

        let anthropic_provider = create_provider(AiProvider::Anthropic, "test_key".to_string());
        assert!(anthropic_provider.is_ok());
    }

    #[test]
    fn test_clean_output_groq() {
        let provider = GroqProvider::new("test_key".to_string()).unwrap();

        let input = "<think>Some thoughts</think>feat: add new feature";
        let expected = "feat: add new feature";
        assert_eq!(provider.clean_output(input), expected);
    }

    #[test]
    fn test_clean_output_anthropic() {
        let provider = AnthropicProvider::new("test_key".to_string()).unwrap();

        let input = "<thinking>Some thoughts</thinking>feat: add new feature";
        let expected = "feat: add new feature";
        assert_eq!(provider.clean_output(input), expected);
    }
}
