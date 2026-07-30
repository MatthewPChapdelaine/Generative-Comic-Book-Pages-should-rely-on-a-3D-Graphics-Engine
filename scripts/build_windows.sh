#!/usr/bin/env bash
set -euo pipefail

# Build windows release. Requires appropriate target toolchain installed.
TARGET=x86_64-pc-windows-gnu
cargo build --release --target "$TARGET"

echo "Built for $TARGET at target/$TARGET/release/pleromic-pipeline.exe"
echo "Use NSIS (makensis) with packaging/installer.nsi to create an installer."
