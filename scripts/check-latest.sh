#!/bin/bash

# Script to check if you have the latest version of ConvCom
# Usage: ./scripts/check-latest.sh

set -e

REPO="topp/convcom"  # Update this with actual repo
API_URL="https://api.github.com/repos/$REPO/releases/latest"

echo "🔍 Checking for latest ConvCom release..."

# Get latest release info
LATEST_VERSION=$(curl -s "$API_URL" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo "❌ Could not fetch latest version from GitHub"
    exit 1
fi

echo "📦 Latest version: $LATEST_VERSION"

# Check if convcom is installed
if command -v convcom &> /dev/null; then
    CURRENT_VERSION=$(convcom --version 2>/dev/null || echo "unknown")
    echo "💻 Your version: $CURRENT_VERSION"
    
    if [[ "$CURRENT_VERSION" == *"$LATEST_VERSION"* ]]; then
        echo "✅ You have the latest version!"
    else
        echo "⬆️  Update available!"
        echo "🔗 Download from: https://github.com/$REPO/releases/latest"
    fi
else
    echo "❌ ConvCom not found in PATH"
    echo "📥 Download from: https://github.com/$REPO/releases/latest"
fi
