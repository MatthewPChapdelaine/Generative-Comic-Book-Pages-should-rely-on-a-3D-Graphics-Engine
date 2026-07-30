#!/usr/bin/env bash
set -euo pipefail

# Builds the project and attempts to create an AppImage if appimagetool is installed.
cargo build --release

APPDIR=dist/AppDir
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
cp target/release/pleromic-pipeline "$APPDIR/usr/bin/"
cp -r assets "$APPDIR/usr/"

if command -v appimagetool >/dev/null 2>&1; then
  appimagetool "$APPDIR"
  echo "AppImage created."
else
  echo "appimagetool not found — AppDir prepared at $APPDIR. Install appimagetool to create the AppImage."
fi
