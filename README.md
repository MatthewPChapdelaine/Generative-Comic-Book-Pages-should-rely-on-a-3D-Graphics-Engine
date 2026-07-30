# Pleromic Pipeline — reference implementation

This repository contains a scaffolded reference implementation of the "Pleromic Pipeline" described in the source document. It provides a minimal Rust library and CLI that "materializes" a panel into a PNG image. The goal is a cross-platform package that can be built and packaged for Windows and Linux.

Quick start (Linux):

```bash
./scripts/build_linux.sh
# then use appimagetool or appimage-builder to create an AppImage from dist/AppDir
```

Quick start (Windows):

```powershell
.
.\scripts\build_windows.sh
# Use NSIS (makensis) to compile packaging\installer.nsi into an installer
```

To run locally:

```bash
cargo run --release -- output.png
```

Notes:
- The included renderer is a placeholder that produces a gradient PNG. Replace `src/gpu` with a `wgpu` implementation for full GPU rendering as described in the document.
- CI workflow builds Linux and Windows release binaries and uploads them as artifacts.
# Generative-Comic-Book-Pages-should-rely-on-a-3D-Graphics-Engine
Generative Comic Book Pages should rely on a 3D Graphics Engine
