# Build VaultX for all platforms
# Requires: cargo install cross

Write-Host "🌍 Building VaultX for all platforms..." -ForegroundColor Cyan

# Check if cross is installed
if (!(Get-Command cross -ErrorAction SilentlyContinue)) {
    Write-Host "Installing cross..." -ForegroundColor Yellow
    cargo install cross
}

$targets = @(
    @{Name="Windows x64"; Target="x86_64-pc-windows-msvc"; Output="vx-windows-x64.exe"},
    @{Name="Linux x64"; Target="x86_64-unknown-linux-gnu"; Output="vx-linux-x64"},
    @{Name="macOS x64"; Target="x86_64-apple-darwin"; Output="vx-macos-x64"},
    @{Name="macOS ARM"; Target="aarch64-apple-darwin"; Output="vx-macos-arm64"}
)

# Create dist directory
New-Item -ItemType Directory -Force -Path "dist" | Out-Null

foreach ($target in $targets) {
    Write-Host "`n📦 Building $($target.Name)..." -ForegroundColor Cyan
    
    # Add target
    rustup target add $target.Target 2>$null
    
    # Build
    if ($target.Target -eq "x86_64-pc-windows-msvc") {
        cargo build --release --target $target.Target
    } else {
        cross build --release --target $target.Target
    }
    
    if ($LASTEXITCODE -eq 0) {
        # Copy to dist
        $ext = if ($target.Output -like "*.exe") { ".exe" } else { "" }
        $source = "target\$($target.Target)\release\vx$ext"
        $dest = "dist\$($target.Output)"
        
        if (Test-Path $source) {
            Copy-Item $source $dest
            $size = (Get-Item $dest).Length / 1MB
            Write-Host "  ✓ $($target.Name): $([math]::Round($size, 2)) MB" -ForegroundColor Green
        }
    } else {
        Write-Host "  ✗ $($target.Name) failed" -ForegroundColor Red
    }
}

Write-Host "`n✅ Build complete! Binaries in dist/" -ForegroundColor Green
Write-Host "`nBinaries:" -ForegroundColor Cyan
Get-ChildItem dist | ForEach-Object {
    $size = $_.Length / 1MB
    Write-Host "  - $($_.Name): $([math]::Round($size, 2)) MB"
}
