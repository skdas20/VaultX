#!/usr/bin/env node

/**
 * VaultX Installation Script
 * 
 * This script detects the user's platform and architecture,
 * then downloads the appropriate pre-built binary from GitHub releases.
 */

const os = require('os');
const path = require('path');
const fs = require('fs');
const https = require('https');
const { execSync } = require('child_process');

const GITHUB_REPO = 'skdas20/VaultX';
// Get version from package.json
const packageJson = require('../package.json');
const RELEASE_TAG = `v${packageJson.version}`; // e.g., 'v0.3.4'
const BINARY_DIR = path.join(__dirname, '..', 'binaries');

// Detect platform and architecture
function getPlatform() {
  const platform = process.platform;
  const arch = process.arch;

  let binaryName = null;
  let downloadUrl = null;

  switch (platform) {
    case 'linux':
      if (arch === 'x64') {
        binaryName = 'vx-x86_64-unknown-linux-gnu';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/${binaryName}`;
      } else {
        console.warn(`⚠️  Linux architecture '${arch}' not explicitly supported. Please build from source.`);
        process.exit(1);
      }
      break;

    case 'darwin':
      if (arch === 'x64') {
        binaryName = 'vx-x86_64-apple-darwin';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/${binaryName}`;
      } else if (arch === 'arm64') {
        binaryName = 'vx-aarch64-apple-darwin';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/${binaryName}`;
      } else {
        console.error(`❌ Unsupported macOS architecture: ${arch}`);
        process.exit(1);
      }
      break;

    case 'win32':
      if (arch === 'x64') {
        binaryName = 'vx-x86_64-pc-windows-msvc.exe';
        downloadUrl = `https://github.com/${GITHUB_REPO}/releases/download/${RELEASE_TAG}/${binaryName}`;
      } else {
        console.error(`❌ Unsupported Windows architecture: ${arch}`);
        process.exit(1);
      }
      break;

    default:
      console.error(`❌ Unsupported platform: ${platform}`);
      process.exit(1);
  }

  return { platform, arch, binaryName, downloadUrl };
}

// Download file from GitHub
function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    // For GitHub releases, we need to follow redirects
    const maxRedirects = 5;
    let redirectCount = 0;

    function download(downloadUrl) {
      if (redirectCount > maxRedirects) {
        reject(new Error('Too many redirects'));
        return;
      }

      console.log(`📥 Downloading from: ${downloadUrl}`);

      https.get(downloadUrl, (response) => {
        if (response.statusCode >= 300 && response.statusCode < 400 && response.headers.location) {
          redirectCount++;
          download(response.headers.location);
          return;
        }

        if (response.statusCode !== 200) {
          reject(new Error(`Download failed: HTTP ${response.statusCode}`));
          return;
        }

        const file = fs.createWriteStream(dest);
        let downloadedBytes = 0;
        const totalBytes = parseInt(response.headers['content-length'], 10);

        response.on('data', (chunk) => {
          downloadedBytes += chunk.length;
          const percent = Math.round((downloadedBytes / totalBytes) * 100);
          process.stdout.write(`\r  Progress: ${percent}%`);
        });

        response.pipe(file);

        file.on('finish', () => {
          file.close();
          console.log('\n✅ Download complete');
          resolve();
        });

        file.on('error', (err) => {
          fs.unlink(dest, () => {}); // Delete the file on error
          reject(err);
        });
      }).on('error', reject);
    }

    download(url);
  });
}

// Main installation function
async function install() {
  try {
    console.log('🔧 VaultX Installation');
    console.log('═════════════════════════════════════════');
    console.log('');

    // Detect platform
    const { platform, arch, binaryName, downloadUrl } = getPlatform();
    console.log(`✓ Detected: ${platform} (${arch})`);
    console.log(`✓ Binary: ${binaryName}`);
    console.log('');

    // Create binaries directory
    if (!fs.existsSync(BINARY_DIR)) {
      fs.mkdirSync(BINARY_DIR, { recursive: true });
    }

    const binaryPath = path.join(BINARY_DIR, binaryName);

    try {
      // Try downloading binary first
      console.log('📦 Downloading binary...');
      await downloadFile(downloadUrl, binaryPath);
    } catch (downloadError) {
      console.warn(`⚠️  Download failed: ${downloadError.message}`);
      console.log('🔄 Attempting to build from source...');
      
      try {
        await buildFromSource(binaryName, platform);
      } catch (buildError) {
        throw new Error(`Download failed and build from source failed: ${buildError.message}`);
      }
    }

    // Make executable (Linux/macOS)
    if (platform !== 'win32') {
      console.log('🔐 Setting permissions...');
      fs.chmodSync(binaryPath, 0o755);
    }

    console.log('');
    console.log('═════════════════════════════════════════');
    console.log('✅ Installation successful!');
    console.log('');
    console.log('🚀 To get started:');
    console.log('  vx --version   # Check version');
    console.log('  vx --help      # Show all commands');
    console.log('  vx init        # Create a new vault');
    console.log('');
    console.log('📖 Learn more: https://github.com/skdas20/VaultX#quickstart');
    console.log('');
  } catch (error) {
    console.error('');
    console.error('❌ Installation failed:', error.message);
    console.error('');
    console.error('💡 Possible solutions:');
    console.error('  1. Check your internet connection');
    console.error('  2. Verify the binary exists for your platform');
    console.error('  3. Try building from source: https://github.com/skdas20/VaultX');
    process.exit(1);
  }
}

// Build from source fallback
function buildFromSource(targetBinaryName, platform) {
  return new Promise((resolve, reject) => {
    try {
      // Check if cargo is available
      try {
        execSync('cargo --version', { stdio: 'ignore' });
      } catch (e) {
        throw new Error('Rust/Cargo is not installed');
      }

      // 1. Try to find bundled source (for npm installed package)
      const bundledSourceDir = path.join(__dirname, '..', 'rust-src');
      const bundledCargoToml = path.join(bundledSourceDir, 'Cargo.toml');
      
      // 2. Try to find parent source (for git repo / local dev)
      const parentSourceDir = path.resolve(__dirname, '..', '..');
      const parentCargoToml = path.join(parentSourceDir, 'Cargo.toml');

      let projectRoot = null;
      let isBundled = false;

      if (fs.existsSync(bundledCargoToml)) {
        console.log('📦 Found bundled source code.');
        projectRoot = bundledSourceDir;
        isBundled = true;
      } else if (fs.existsSync(parentCargoToml)) {
        console.log('📂 Found local source code.');
        projectRoot = parentSourceDir;
      } else {
        throw new Error(`Cargo.toml not found. Source code not available for fallback build.`);
      }

      console.log('🔨 Building release binary with Cargo...');
      
      // Build command
      // If bundled, we are at the workspace root equivalent
      execSync('cargo build --release -p vx-cli', { 
        cwd: projectRoot, 
        stdio: 'inherit' 
      });

      // Locate the built binary
      const builtBinaryName = platform === 'win32' ? 'vx.exe' : 'vx';
      const sourcePath = path.join(projectRoot, 'target', 'release', builtBinaryName);

      if (!fs.existsSync(sourcePath)) {
        throw new Error(`Built binary not found at ${sourcePath}`);
      }

      // Copy to destination
      const destPath = path.join(BINARY_DIR, targetBinaryName);
      fs.copyFileSync(sourcePath, destPath);
      console.log(`✅ Binary built and copied to ${destPath}`);
      resolve();

    } catch (err) {
      reject(err);
    }
  });
}

// Run installation if this script is executed directly
if (require.main === module) {
  install();
}

module.exports = { getPlatform, downloadFile };
