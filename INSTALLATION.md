# ConvCom Installation Guide

A Rust-based AI-powered conventional commit message generator using Groq API.

## Prerequisites

- **AI Provider API Key**: Get API key from one or both providers:
  - **Groq**: Free API key from [console.groq.com](https://console.groq.com) (fast, affordable)
  - **Anthropic**: API key from [console.anthropic.com](https://console.anthropic.com) (premium quality)
- **Git**: Ensure git is installed and you're working in a git repository

## Installation Options

### Option 1: Install from Source (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd convcom

# Install globally using Cargo
cargo install --path .

# The 'convcom' command is now available globally
convcom --help
```

### Option 2: Install Directly from Git

```bash
# Install directly from git repository
cargo install --git <repository-url>
```

### Option 3: Download Pre-compiled Binaries ⭐ **EASIEST**

**Quick and easy installation without Rust toolchain required!**

1. **Download**: Go to the [Releases page](https://github.com/topp/convcom/releases) and download the appropriate binary:
   - **🐧 Linux**: `convcom-linux-x86_64.tar.gz`
   - **🪟 Windows**: `convcom-windows-x86_64.exe.zip`
   - **🍎 macOS (Intel)**: `convcom-macos-x86_64.tar.gz`
   - **🍎 macOS (Apple Silicon)**: `convcom-macos-aarch64.tar.gz`

2. **Extract and Install**:

   **Linux/macOS:**
   ```bash
   # Extract the downloaded file
   tar -xzf convcom-*.tar.gz
   
   # Make executable
   chmod +x convcom
   
   # Move to your PATH (optional but recommended)
   sudo mv convcom /usr/local/bin/
   
   # Or for user-local installation:
   mkdir -p ~/.local/bin
   mv convcom ~/.local/bin/
   # Make sure ~/.local/bin is in your PATH
   ```

   **Windows:**
   ```powershell
   # Extract the zip file to a folder
   # Add the folder to your PATH environment variable
   # Or run convcom.exe directly from the extracted location
   ```

3. **Verify Installation**:
   ```bash
   convcom --help
   ```

> **💡 Tip**: This method is perfect if you just want to use ConvCom without setting up the Rust development environment!

### Option 4: Build and Run Locally

```bash
# Clone and enter directory
git clone <repository-url>
cd convcom

# For clean output without cargo messages:
cargo run --quiet -- [OPTIONS]

# Or build and run the binary directly:
cargo build --release
./target/release/convcom [OPTIONS]
```

## Configuration

### Step 1: Set up your AI Provider API Key(s)

Choose one of these methods and providers:

**Method A: Environment Variables (Recommended)**

Option 1 - Groq (Fast & Affordable):
```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export GROQ_API_KEY="your_groq_api_key_here"
```

Option 2 - Anthropic Claude (Premium Quality):
```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export ANTHROPIC_API_KEY="your_anthropic_api_key_here"
```

Option 3 - Both Providers (Maximum Flexibility):
```bash
# Use both providers for maximum model selection
export GROQ_API_KEY="your_groq_api_key_here"
export ANTHROPIC_API_KEY="your_anthropic_api_key_here"
```

**Method B: Config File**
```bash
# Create config directory
mkdir -p ~/.config/conv_commit_ai

# Create config file with one or both API keys
cat > ~/.config/conv_commit_ai/.env.commits << EOF
GROQ_API_KEY=your_groq_api_key_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
EOF
```

### Step 2: Verify Installation

```bash
# Test the installation
convcom --help

# Test with a simple change (in any git repo)
echo "# Test" >> README.md
git add README.md
convcom
```

## Usage

### Basic Usage

```bash
# Stage your changes
git add .

# Generate commit message
convcom

# Use the generated message
git commit -m "$(convcom)"
```

### Advanced Usage

```bash
# Groq models (fast, cost-effective)
convcom --model llama-3.3-70b-versatile    # Default, excellent quality
convcom --model llama-3.1-8b-instant       # Fastest inference
convcom --model qwen-qwq-32b               # Best reasoning

# Anthropic models (premium quality)
convcom --model claude-3-5-sonnet-20241022 # Top-tier reasoning
convcom --model claude-3-5-haiku-20241022  # Fast Claude model
convcom --model claude-3-opus-20240229     # Maximum capability

# Add focus/guidance for any model
convcom --model claude-3-5-sonnet-20241022 --focus "emphasize security"
convcom --model llama-3.1-8b-instant --focus "keep it concise"

# Compare perspectives from different providers
convcom --model llama-3.3-70b-versatile    # Groq perspective
convcom --model claude-3-5-sonnet-20241022 # Claude perspective
```

### Available Models

**Groq Models (Fast & Affordable)**:
- `llama-3.3-70b-versatile` (default) - Excellent balance
- `llama-3.1-8b-instant` - Fastest inference
- `gemma2-9b-it` - Great quality/speed ratio
- `qwen-qwq-32b` - Best reasoning capabilities
- `deepseek-r1-distill-llama-70b` - Advanced reasoning
- And many more... (use `convcom --help` to see all)

**Anthropic Models (Premium Quality)**:
- `claude-3-5-sonnet-20241022` - Top-tier reasoning and coding
- `claude-3-5-haiku-20241022` - Fast Claude model
- `claude-3-opus-20240229` - Maximum capability model
- `claude-3-sonnet-20240229` - Balanced performance
- `claude-3-haiku-20240307` - Efficient model

### Integration with Git Workflows

**Create Git Alias (Recommended)**
```bash
# Add to your git config
git config --global alias.aic '!convcom'

# Now you can use:
git add .
git aic                    # Generate message
git commit -m "$(git aic)" # Commit with generated message
```

**Shell Function for Easy Commits**
```bash
# Add to your .bashrc/.zshrc
aicommit() {
    git add .
    local message=$(convcom)
    echo "Generated commit message:"
    echo "$message"
    echo ""
    read -p "Use this message? (y/N): " confirm
    if [[ $confirm == [yY] ]]; then
        git commit -m "$message"
    fi
}

# Usage:
aicommit
```

**VS Code Integration**
```json
// Add to VS Code tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "AI Commit Message",
            "type": "shell",
            "command": "convcom",
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            }
        }
    ]
}
```

## Troubleshooting

### Common Issues

**"No staged files found"**
```bash
# Make sure you have staged changes
git status
git add <files>
```

**"GROQ_API_KEY not found"**
```bash
# Verify your API key is set
echo $GROQ_API_KEY

# Or check config file exists
cat ~/.config/conv_commit_ai/.env.commits
```

**"Not in a git repository"**
```bash
# Ensure you're in a git repository
git status

# Initialize if needed
git init
```

**Cargo build messages in output**
```bash
# Use --quiet flag when running with cargo
cargo run --quiet

# Or install globally to avoid cargo messages
cargo install --path .
convcom  # Clean output
```

### Clean Output for Copy/Paste

When installed globally, `convcom` produces only the commit message:

```bash
# Clean output (after global installation)
convcom
# Output: feat: add new feature
#
# * Added new functionality for user authentication
# * Implemented secure session management
# * Updated API endpoints for better performance

# For use in commands:
git commit -m "$(convcom)"
```

## Updating

```bash
# Update from git
cargo install --git <repository-url> --force

# Or if installed from source
cd convcom
git pull
cargo install --path . --force
```

## Uninstalling

```bash
# Remove the binary
cargo uninstall convcom

# Optionally remove config
rm -rf ~/.config/conv_commit_ai
```

## Support

For issues, feature requests, or contributions, please visit the [GitHub repository](repository-url).
