use clap::Parser;
use anyhow::Result;

mod config;
mod error;
mod models;
mod prompt;
mod services;

use config::settings::Config;
use models::providers::ModelName;
use prompt::builder::PromptBuilder;
use services::{ai_service::AiService, git_service::GitService};

#[derive(Parser)]
#[command(name = "convcom")]
#[command(about = "Generate conventional commit messages using AI")]
#[command(version = "0.1.0")]
struct Cli {
    /// The AI model to use
    #[arg(long, value_enum, default_value_t = ModelName::default())]
    model: ModelName,
    
    /// Focus message to guide AI attention
    #[arg(long, short)]
    focus: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Load configuration
    let config = match Config::load() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            eprintln!();
            eprintln!("Please set your GROQ_API_KEY:");
            eprintln!("1. Get a free API key from: https://console.groq.com");
            eprintln!("2. Set it in your environment: export GROQ_API_KEY=\"your_key_here\"");
            eprintln!("3. Or create config file: ~/.config/conv_commit_ai/.env.commits");
            std::process::exit(1);
        }
    };

    // Create AI service with available providers
    let ai_service = match AiService::new(config.groq_api_key, config.anthropic_api_key) {
        Ok(service) => service,
        Err(e) => {
            eprintln!("AI service initialization error: {}", e);
            eprintln!();
            eprintln!("Please configure at least one AI provider:");
            eprintln!("1. Groq: Set GROQ_API_KEY (get free key from console.groq.com)");
            eprintln!("2. Anthropic: Set ANTHROPIC_API_KEY (get key from console.anthropic.com)");
            eprintln!("3. Or create config file: ~/.config/conv_commit_ai/.env.commits");
            std::process::exit(1);
        }
    };

    // Validate that the selected model is available
    if !ai_service.has_provider(cli.model.provider()) {
        eprintln!("Error: Model '{}' requires {} provider, but no API key is configured.", 
                  cli.model, cli.model.provider());
        eprintln!();
        eprintln!("Available providers: {:?}", ai_service.available_providers());
        eprintln!("Configure the required API key or choose a different model.");
        std::process::exit(1);
    }

    // Create Git service and get diff content
    let git_service = match GitService::new() {
        Ok(service) => service,
        Err(e) => {
            eprintln!("Git error: {}", e);
            eprintln!("Make sure you're in a git repository with staged changes.");
            std::process::exit(1);
        }
    };

    let diff_content = match git_service.build_diff_content() {
        Ok(content) => {
            if content.trim().is_empty() {
                eprintln!("No staged changes found.");
                eprintln!("Use 'git add <files>' to stage changes for commit.");
                std::process::exit(1);
            }
            content
        }
        Err(e) => {
            eprintln!("Error getting git diff: {}", e);
            std::process::exit(1);
        }
    };

    // Create prompt builder
    let prompt_builder = match PromptBuilder::new() {
        Ok(builder) => builder,
        Err(e) => {
            eprintln!("Template error: {}", e);
            std::process::exit(1);
        }
    };

    // Build the complete prompt using the template system
    let prompt = match prompt_builder.build_prompt(&diff_content, cli.focus.as_deref()) {
        Ok(prompt) => prompt,
        Err(e) => {
            eprintln!("Error building prompt: {}", e);
            std::process::exit(1);
        }
    };

    // Generate commit message (silently for clean output)
    match ai_service.generate_commit_message(prompt, cli.model).await {
        Ok(commit_message) => {
            println!("{}", commit_message);
        }
        Err(e) => {
            eprintln!("Error generating commit message: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
