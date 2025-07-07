# ConvCom

A **Rust-based AI-powered conventional commit message generator** that automatically creates high-quality commit messages based on your staged Git changes using Groq's advanced language models.

## Features

- **AI-Powered Analysis**: Uses Groq's advanced language models to understand your code changes
- **Conventional Commits**: Generates messages following the [Conventional Commits v1.0.0](https://www.conventionalcommits.org/) specification
- **Multiple Models**: Support for various Groq models including Llama, Gemma, Qwen, and more
- **Smart Git Integration**: Analyzes real git repositories and staged changes
- **Focus Mode**: Use the `--focus` option to guide the AI's attention to specific aspects
- **Comprehensive Output**: Provides structured commit messages with bullet-point summaries
- **Fast & Efficient**: Native Rust performance with async HTTP client

> **Quick Recommendation**: For the best experience, use the default `llama-3.3-70b-versatile` model via Groq - it provides excellent quality and speed. Note that Groq has usage limits on their free tier - check [their limits page](https://console.groq.com/dashboard/limits) for details.

## Quick Start

### 1. Installation

```bash
# Install globally from source
git clone <repository-url>
cd convcom
cargo install --path .
```

**For detailed installation options, see [INSTALLATION.md](INSTALLATION.md)**

**Quick Install**: Download pre-compiled binaries from our [Releases page](https://github.com/topp/convcom/releases) - no Rust required!

### 2. Configuration

```bash
# Option 1: Use Groq (fast, affordable)
export GROQ_API_KEY="your_groq_api_key_here"

# Option 2: Use Anthropic Claude (premium quality)  
export ANTHROPIC_API_KEY="your_anthropic_api_key_here"

# Option 3: Use both providers for maximum flexibility
export GROQ_API_KEY="your_groq_key"
export ANTHROPIC_API_KEY="your_anthropic_key"
```

### 3. Usage

```bash
# Stage your changes
git add .

# Generate commit message
convcom

# Use it in your commit
git commit -m "$(convcom)"
```

## Example Output

For a typical code change, the tool generates structured commit messages like:

```
feat(api): add user authentication system

* Implement JWT-based authentication with secure token generation
* Add login and logout endpoints with comprehensive rate limiting
* Create middleware for automatic token validation and user context
* Update user model with secure password hashing using bcrypt
* Add comprehensive error handling for authentication failures
* Include session management with configurable expiration times
```

## Advanced Usage

```bash
# Groq models (fast, cost-effective) - RECOMMENDED
convcom --model llama-3.3-70b-versatile    # Default, excellent quality & speed
convcom --model llama-3.1-8b-instant       # Fastest
convcom --model qwen-qwq-32b               # Best reasoning

# Anthropic models (premium quality)
convcom --model claude-sonnet-4-20250514   # Latest Claude 4 Sonnet
convcom --model claude-3-5-sonnet-20241022 # Top-tier reasoning
convcom --model claude-3-5-haiku-20241022  # Fast Claude model
convcom --model claude-3-opus-20240229     # Maximum capability

# Add focus/guidance for any model
convcom --model claude-3-5-sonnet-20241022 --focus "emphasize security"
convcom --model llama-3.1-8b-instant --focus "keep it concise"

# Compare providers for the same change
convcom --model llama-3.3-70b-versatile    # Groq perspective
convcom --model claude-3-5-sonnet-20241022 # Claude perspective
```

## AI Provider Support

ConvCom supports multiple AI providers for maximum flexibility:

### Groq (Fast & Affordable) - **HIGHLY RECOMMENDED**
- **Models**: Llama 3.3 70B, Llama 3.1 8B, Gemma2 9B, Qwen QWQ 32B, and more
- **Default Model**: `llama-3.3-70b-versatile` - **Optimal balance of quality and speed**
- **Speed**: Very fast inference with excellent performance
- **Cost**: Free tier available with usage limits (see [limits page](https://console.groq.com/dashboard/limits)), low cost for additional usage
- **Quality**: Exceptional results for commit message generation
- **Setup**: `export GROQ_API_KEY="your_key_here"`

> **Recommendation**: The default Llama 3.3 70B Versatile model via Groq provides the best combination of speed, quality, and cost-effectiveness for commit message generation. Highly recommended for daily use!

### Anthropic Claude (Premium Quality)  
- **Models**: Claude 4 Sonnet, Claude 3.5 Sonnet, Claude 3.5 Haiku, Claude 3 Opus
- **Quality**: Exceptional reasoning and code understanding
- **Features**: Advanced thinking capabilities, state-of-the-art performance
- **Latest**: Claude 4 Sonnet offers the most advanced AI capabilities
- **Setup**: `export ANTHROPIC_API_KEY="your_key_here"`

### Mixed Provider Usage
```bash
# Use both providers (set both API keys)
export GROQ_API_KEY="your_groq_key"
export ANTHROPIC_API_KEY="your_anthropic_key"

# Choose specific models
convcom --model llama-3.3-70b-versatile    # Groq (recommended)
convcom --model claude-sonnet-4-20250514   # Latest Claude 4 Sonnet
```

## Integration Ideas

**Git Alias (Recommended)**
```bash
git config --global alias.aic '!convcom'
# Usage: git aic
```

**Shell Function for Interactive Commits**
```bash
aicommit() {
    git add .
    local message=$(convcom)
    echo "Generated: $message"
    read -p "Use this message? (y/N): " confirm
    [[ $confirm == [yY] ]] && git commit -m "$message"
}
```

## How It Works

1. **Change Detection**: Analyzes staged Git changes using `git2` library
2. **Smart Processing**: 
   - For new files: Includes complete content analysis
   - For modified files: Extracts specific additions and deletions  
   - For deleted files: Notes the removal
3. **AI Analysis**: Sends structured diff to Groq's language model with optimized prompts
4. **Message Generation**: Creates properly formatted commit message with type, scope, description, and detailed body

## Development

```bash
# Clone and setup
git clone <repository-url>
cd convcom

# Run in development
cargo run -- --help

# For clean output (no cargo messages):
cargo run --quiet

# Build release version
cargo build --release
```

## Requirements

- **Rust**: 1.88+ (2024 edition)
- **Git**: Any recent version
- **Groq API Key**: Available at [console.groq.com](https://console.groq.com) (free tier with limits)

## Troubleshooting

**Clean Output for Copy/Paste**: After global installation, `convcom` produces only the commit message without build logs.

**Common Issues**:
- "No staged files found" → `git add <files>`
- "GROQ_API_KEY not found" → Set environment variable
- "Not in a git repository" → Run in a git repo

**For detailed troubleshooting, see [INSTALLATION.md](INSTALLATION.md)**

## License

This project is maintained by Thomas Oppelt (info@deepagents.ai).

---

**Transform your commit messages with AI-powered conventional commits!**
