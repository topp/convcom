use convcom::{Config, AiService, GitService, ModelName};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Testing AI Service...");
    
    // Try to load config
    match Config::load() {
        Ok(config) => {
            println!("✅ Configuration loaded successfully");
            
            // Test AI service creation
            match AiService::new(config.groq_api_key, config.anthropic_api_key) {
                Ok(ai_service) => {
                    println!("✅ AI Service created successfully");
                    
                    // Test with a simple prompt
                    let test_prompt = r#"Generate a conventional commit message for:
MODIFIED: README.md
+ # New feature added
+ This adds a new awesome feature."#;
                    
                    println!("🤖 Testing AI call with model: {}", ModelName::default());
                    
                    match ai_service.generate_commit_message(
                        test_prompt.to_string(), 
                        ModelName::default()
                    ).await {
                        Ok(message) => {
                            println!("✅ AI Service working! Generated message:");
                            println!("---");
                            println!("{}", message);
                            println!("---");
                            
                            // Test Git service if we're in a git repo
                            println!("\n🔧 Testing Git Service...");
                            match GitService::new() {
                                Ok(git_service) => {
                                    println!("✅ Git Service created successfully");
                                    
                                    match git_service.get_staged_files() {
                                        Ok(staged_files) => {
                                            println!("✅ Found {} staged files", staged_files.len());
                                            for file in &staged_files {
                                                let status = git_service.get_file_status(file).unwrap_or('?');
                                                println!("   - {} ({})", file, status);
                                            }
                                            
                                            match git_service.build_diff_content() {
                                                Ok(diff) => {
                                                    println!("✅ Diff content generated ({} chars)", diff.len());
                                                    if diff.len() < 500 {
                                                        println!("Diff preview:\n{}", diff);
                                                    }
                                                }
                                                Err(e) => println!("⚠️  Could not build diff: {}", e),
                                            }
                                        }
                                        Err(e) => {
                                            println!("ℹ️  No staged files: {}", e);
                                            println!("   (This is normal if nothing is staged)");
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("⚠️  Git Service: {}", e);
                                    println!("   (This is normal if not in a git repository)");
                                }
                            }
                        }
                        Err(e) => {
                            println!("❌ AI Service call failed: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("❌ Failed to create AI Service: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Configuration error: {}", e);
            println!("💡 Make sure to set GROQ_API_KEY environment variable");
            println!("   Or create ~/.config/conv_commit_ai/.env.commits with:");
            println!("   GROQ_API_KEY=your_key_here");
        }
    }
    
    Ok(())
}
