#!/bin/bash
set -e

REPO="kevin-zhangwen/rustgrep"
BINARY="rustgrep"
INSTALL_DIR="/usr/local/bin"

echo "🚀 Installing rustgrep..."

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case $ARCH in
  x86_64|amd64)
    ARCH="x86_64"
    ;;
  arm64|aarch64)
    ARCH="aarch64"
    ;;
esac

# Check if cargo is available (prefer local build)
if command -v cargo &> /dev/null; then
  echo "📦 Building from source with cargo..."

  TEMP_DIR=$(mktemp -d)
  git clone "https://github.com/$REPO.git" "$TEMP_DIR/rustgrep"
  cd "$TEMP_DIR/rustgrep"
  cargo build --release

  echo "📁 Installing to $INSTALL_DIR..."
  sudo cp target/release/$BINARY "$INSTALL_DIR/"

  rm -rf "$TEMP_DIR"
else
  echo "📥 Downloading prebuilt binary..."

  # Get latest version
  VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')

  if [ -z "$VERSION" ]; then
    VERSION="0.1.0"
  fi

  DOWNLOAD_URL="https://github.com/$REPO/releases/download/v$VERSION/$BINARY-$OS-$ARCH"

  TEMP_FILE=$(mktemp)
  curl -L -o "$TEMP_FILE" "$DOWNLOAD_URL"
  chmod +x "$TEMP_FILE"

  echo "📁 Installing to $INSTALL_DIR..."
  sudo mv "$TEMP_FILE" "$INSTALL_DIR/$BINARY"
fi

echo "✅ rustgrep installed successfully!"
echo ""
echo "Usage: rustgrep [options] <pattern> <path>"
echo "Run 'rustgrep --help' for more information."
