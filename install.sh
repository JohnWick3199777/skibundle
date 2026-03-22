#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="ski"

echo "Building skilib in release mode..."
cargo build --release

BINARY_PATH="$(pwd)/target/release/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
  echo "error: binary not found at $BINARY_PATH" >&2
  exit 1
fi

mkdir -p "$INSTALL_DIR"
echo "Installing $BINARY_NAME → $INSTALL_DIR/$BINARY_NAME"

if [ -w "$INSTALL_DIR" ]; then
  cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
else
  sudo cp "$BINARY_PATH" "$INSTALL_DIR/$BINARY_NAME"
fi

# Warn if INSTALL_DIR is not on PATH
if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
  echo ""
  echo "  NOTE: $INSTALL_DIR is not in your PATH."
  echo "  Add this to your ~/.zshrc or ~/.bashrc:"
  echo "    export PATH=\"$INSTALL_DIR:\$PATH\""
  echo ""
fi

echo "Done. Run: ski --help"
