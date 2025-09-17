$ErrorActionPreference = "Stop"

Set-Location -Path "builds"
Get-ChildItem -Filter "*.msi" | Remove-Item -Force

Set-Location -Path ".."
npm run tauri build

$msiPath = Get-ChildItem "../target/release/bundle/msi" -Filter "client-ui_*_x64_en-US.msi" |
    ForEach-Object {
        if ($_ -match 'client-ui_(\d+\.\d+\.\d+)_x64_en-US\.msi') {
            [PSCustomObject]@{
                File    = $_
                Version = [version]$matches[1]
            }
        }
    } |
    Sort-Object Version -Descending |
    Select-Object -First 1

if ($null -eq $msiPath) {
    Write-Error "No MSI package found!"
    exit 1
}

Write-Host "Using client-ui version:" $msiPath.Version

Copy-Item -Path $msiPath.File.FullName -Destination ("builds/" + $msiPath.File.Name) -Force
