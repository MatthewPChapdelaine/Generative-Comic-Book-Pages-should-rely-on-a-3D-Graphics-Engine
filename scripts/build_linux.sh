#!/usr/bin/env bash
set -euo pipefail

# Build release binary for Linux and create a portable AppDir for AppImage
cargo build --release

OUT_BIN=target/release/pleromic-pipeline
PKG_DIR=dist/AppDir
rm -rf "$PKG_DIR"
mkdir -p "$PKG_DIR/usr/bin"
cp "$OUT_BIN" "$PKG_DIR/usr/bin/pleromic-pipeline"
cp -r assets "$PKG_DIR/usr/"

echo "AppDir prepared at $PKG_DIR. Use appimagetool or appimage-builder to create AppImage."
