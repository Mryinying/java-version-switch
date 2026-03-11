#!/bin/bash
set -e

INSTALL_DIR="/usr/local/bin"
JVS_DIR="$HOME/.jvs"
SHELL_RC=""

# Detect shell config file
case "$SHELL" in
  */zsh)  SHELL_RC="$HOME/.zshrc" ;;
  */bash) SHELL_RC="$HOME/.bashrc" ;;
esac

echo "🔧 Installing JVS (Java Version Switch)..."

# Check Rust toolchain
if ! command -v cargo &>/dev/null; then
  echo "📦 Rust not found, installing via rustup..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
fi

# Build
echo "🔨 Building..."
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$SCRIPT_DIR"
cargo build --release

# Install binary
echo "📥 Installing jvs to $INSTALL_DIR..."
sudo cp target/release/jvs "$INSTALL_DIR/jvs"

# Create jvs dir
mkdir -p "$JVS_DIR"

# Configure shell
JVS_MARKER="# JVS - Java Version Switch"
if [ -n "$SHELL_RC" ] && ! grep -q "$JVS_MARKER" "$SHELL_RC" 2>/dev/null; then
  echo "🐚 Adding JVS config to $SHELL_RC..."
  cat >> "$SHELL_RC" << 'EOF'

# JVS - Java Version Switch
export JVS_CONFIG="$HOME/.jvs/config.json"
if [ -f "$HOME/.jvs/env" ]; then
  source "$HOME/.jvs/env"
fi
jvs() {
  command jvs "$@"
  if [ "$1" = "use" ] && [ -f "$HOME/.jvs/env" ]; then
    source "$HOME/.jvs/env"
  fi
}
EOF
fi

echo ""
echo "✅ JVS installed successfully!"
echo "   Run 'source $SHELL_RC' or open a new terminal to start using jvs."
echo "   Try: jvs list"
