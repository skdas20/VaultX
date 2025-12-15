@echo off
REM VaultX Build Script for All Platforms (Windows)
REM Run with: build-all.bat

setlocal enabledelayedexpansion

echo.
echo ===============================================
echo  VaultX Multi-Platform Build
echo ===============================================
echo.

REM Check if cargo is installed
cargo --version >nul 2>&1
if errorlevel 1 (
    echo [ERROR] Cargo not found!
    echo Please install Rust from https://rustup.rs/
    pause
    exit /b 1
)

echo [OK] Cargo found

REM Create dist directory
if not exist "dist" mkdir dist

echo.
echo [1/4] Building for Windows x64...
cargo build --release --target x86_64-pc-windows-msvc
if errorlevel 1 (
    echo [ERROR] Windows build failed!
    goto :error
)
copy target\release\vx.exe dist\vx-windows-x64.exe
echo [OK] Windows x64 binary created

echo.
echo [2/4] Building for Linux x64 (via cross)...
where cross >nul 2>&1
if errorlevel 1 (
    echo [INFO] Installing cross...
    cargo install cross
)

rustup target add x86_64-unknown-linux-gnu
cross build --release --target x86_64-unknown-linux-gnu
if errorlevel 1 (
    echo [WARNING] Linux build failed. Skipping...
) else (
    copy target\x86_64-unknown-linux-gnu\release\vx dist\vx-linux-x64
    echo [OK] Linux x64 binary created
)

echo.
echo [3/4] Building for macOS x64...
rustup target add x86_64-apple-darwin
cross build --release --target x86_64-apple-darwin
if errorlevel 1 (
    echo [INFO] macOS x64 build requires macOS. Use GitHub Actions for automated builds.
) else (
    copy target\x86_64-apple-darwin\release\vx dist\vx-macos-x64
    echo [OK] macOS x64 binary created
)

echo.
echo [4/4] Building for macOS ARM64 (M1/M2)...
rustup target add aarch64-apple-darwin
cross build --release --target aarch64-apple-darwin
if errorlevel 1 (
    echo [INFO] macOS ARM64 build requires macOS. Use GitHub Actions for automated builds.
) else (
    copy target\aarch64-apple-darwin\release\vx dist\vx-macos-arm64
    echo [OK] macOS ARM64 binary created
)

echo.
echo ===============================================
echo  Build Complete!
echo ===============================================
echo.
echo Available binaries in dist\:
dir /B dist\
echo.
pause
exit /b 0

:error
echo.
echo Build failed! Check the error above.
pause
exit /b 1
