# Generates SHA-256 checksums for built binaries in src-tauri/target/release
$ErrorActionPreference = "Stop"

$releaseDir = Join-Path $PSScriptRoot "..\src-tauri\target\release"
$portableExe = Join-Path $releaseDir "synchro.exe"
$nsisDir = Join-Path $releaseDir "bundle\nsis"
$outputFile = Join-Path $PSScriptRoot "..\SHA256SUMS.txt"

if (Test-Path $outputFile) {
    Remove-Item $outputFile -Force
}

$filesToHash = @()
if (Test-Path $portableExe) {
    $filesToHash += Get-Item $portableExe
}
if (Test-Path $nsisDir) {
    $filesToHash += Get-ChildItem -Path $nsisDir -Filter *.exe
}

if ($filesToHash.Count -eq 0) {
    Write-Warning "No release binaries found. Run 'npm run build' or 'npm run build:portable' first."
    exit 1
}

Write-Host "Calculating SHA-256 hashes..."
foreach ($file in $filesToHash) {
    $hash = (Get-FileHash -Path $file.FullName -Algorithm SHA256).Hash.ToLower()
    $line = "$hash  $($file.Name)"
    Write-Host $line
    $line | Out-File -Append -Encoding ascii $outputFile
}

Write-Host "`nHashes written to $(Resolve-Path $outputFile)"
