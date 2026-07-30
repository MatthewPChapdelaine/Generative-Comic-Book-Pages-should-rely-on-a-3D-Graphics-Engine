; NSIS installer script (packaging/installer.nsi)
; Requires NSIS / makensis to build on Windows host

OutFile "pleromic-pipeline-installer.exe"
InstallDir "$PROGRAMFILES\\Pleromic Pipeline"
RequestExecutionLevel admin

Page directory
Page instfiles

Section "Install"
  SetOutPath "$INSTDIR"
  File ".\\target\\x86_64-pc-windows-gnu\\release\\pleromic-pipeline.exe"
  CreateShortCut "$DESKTOP\\Pleromic Pipeline.lnk" "$INSTDIR\\pleromic-pipeline.exe"
SectionEnd
