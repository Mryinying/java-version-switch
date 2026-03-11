#!/bin/bash
set -e

INSTALL_DIR="/usr/local/bin"
JVS_DIR="$HOME/.jvs"

# Detect shell config file
case "$SHELL" in
  */zsh)  SHELL_RC="$HOME/.zshrc" ;;
  */bash) SHELL_RC="$HOME/.bashrc" ;;
esac

echo "🗑  Uninstalling JVS (Java Version Switch)..."

# Remove binary
if [ -f "$INSTALL_DIR/jvs" ]; then
  echo "📦 Removing $INSTALL_DIR/jvs..."
  sudo rm -f "$INSTALL_DIR/jvs"
fi

# Remove config directory
if [ -d "$JVS_DIR" ]; then
  echo "📁 Removing $JVS_DIR..."
  rm -rf "$JVS_DIR"
fi

# Clean shell config
if [ -n "$SHELL_RC" ] && [ -f "$SHELL_RC" ]; then
  if grep -q "# JVS - Java Version Switch" "$SHELL_RC" 2>/dev/null; then
    echo "🐚 Cleaning JVS config from $SHELL_RC..."
    sed -i '' '/# JVS - Java Version Switch/,/^}$/d' "$SHELL_RC"
    # Remove possible trailing blank lines left behind
    sed -i '' -e :a -e '/^\n*$/{$d;N;ba' -e '}' "$SHELL_RC"
  fi
fi

echo ""
echo "✅ JVS has been uninstalled."
echo "   Run 'source $SHELL_RC' or open a new terminal to complete cleanup."
