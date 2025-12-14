# VaultX Build Script for Windows
# Run with: .\build.ps1

Write-Host "🔨 Building VaultX..." -ForegroundColor Cyan

# Check if cargo is installed
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Cargo not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

Write-Host "✓ Cargo found" -ForegroundColor Green

# Build release version
Write-Host "`n📦 Building release version..." -ForegroundColor Cyan
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✅ Build successful!" -ForegroundColor Green
    
    # Get binary size
    $binaryPath = "target\release\vx.exe"
    if (Test-Path $binaryPath) {
        $size = (Get-Item $binaryPath).Length / 1MB
        Write-Host "📊 Binary size: $([math]::Round($size, 2)) MB" -ForegroundColor Cyan
        Write-Host "📍 Location: $binaryPath" -ForegroundColor Cyan
        
        # Test the binary
        Write-Host "`n🧪 Testing binary..." -ForegroundColor Cyan
        & $binaryPath --version
        
        Write-Host "`n✨ Ready to use!" -ForegroundColor Green
        Write-Host "Run: .\target\release\vx.exe --help" -ForegroundColor Yellow
    }
} else {
    Write-Host "`n❌ Build failed!" -ForegroundColor Red
    exit 1
}
