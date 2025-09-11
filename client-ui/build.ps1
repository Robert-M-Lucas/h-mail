$ErrorActionPreference = "Stop"

Set-Location -Path "builds"

Get-ChildItem -Filter "*.msi" | Remove-Item -Force

Set-Location -Path ".."

npm run tauri build

Copy-Item -Path "../target/release/bundle/msi/client-ui_0.1.0_x64_en-US.msi" -Destination "builds/client-ui_0.1.0_x64_en-US.msi" -Force

