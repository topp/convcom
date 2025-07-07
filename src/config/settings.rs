use crate::error::{ConvComError, Result};
use std::env;
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub groq_api_key: Option<String>,
    pub anthropic_api_key: Option<String>,
}

impl Config {
    /// Load configuration from environment and config files
    pub fn load() -> Result<Self> {
        // Try to load from the config file first
        let config_dir = Self::get_config_dir()?;
        let env_file = config_dir.join(".env.commits");
        
        if env_file.exists() {
            dotenvy::from_path(&env_file)
                .map_err(|e| ConvComError::ConfigError(format!("Failed to load config file: {}", e)))?;
        }

        // Get API keys from environment (both are optional)
        let groq_api_key = env::var("GROQ_API_KEY").ok()
            .filter(|key| !key.is_empty());
            
        let anthropic_api_key = env::var("ANTHROPIC_API_KEY").ok()
            .filter(|key| !key.is_empty());

        // At least one API key must be provided
        if groq_api_key.is_none() && anthropic_api_key.is_none() {
            return Err(ConvComError::ConfigError(
                "At least one API key must be provided. Set GROQ_API_KEY and/or ANTHROPIC_API_KEY.".to_string()
            ));
        }

        Ok(Config {
            groq_api_key,
            anthropic_api_key,
        })
    }

    /// Get the configuration directory path
    fn get_config_dir() -> Result<PathBuf> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| ConvComError::ConfigError("Could not determine home directory".to_string()))?;

        Ok(home_dir.join(".config").join("conv_commit_ai"))
    }
}
