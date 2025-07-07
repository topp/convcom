pub mod config;
pub mod error;
pub mod models;
pub mod prompt;
pub mod services;

pub use config::settings::Config;
pub use error::{ConvComError, Result};
pub use models::providers::{AiProvider, ModelName};
pub use prompt::builder::PromptBuilder;
pub use services::{ai_service::AiService, git_service::GitService};
