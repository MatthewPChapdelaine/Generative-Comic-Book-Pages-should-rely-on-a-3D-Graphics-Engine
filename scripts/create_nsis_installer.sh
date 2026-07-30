#!/usr/bin/env bash
set -euo pipefail

# This script is a helper to invoke makensis if available (typically on Windows).
if command -v makensis >/dev/null 2>&1; then
  makensis packaging/installer.nsi
  echo "NSIS installer created: pleromic-pipeline-installer.exe"
else
  echo "makensis not found. Run this on a Windows host with NSIS installed, or use Wine+makensis."
fi
